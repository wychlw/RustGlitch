use clap::{Parser, ValueEnum, builder::ValueParser};
use pyo3::pyclass;
use serde_json::Value;
use std::{
    error::Error,
    fs,
    path::Path,
    path::PathBuf,
    sync::{LazyLock, RwLock},
};

pub use crate::pipeline::SynMutateParams;

use crate::{
    fuzz::fuzzbase::Fuzzer,
    pipeline::FuzzerJob,
};

const PIPELINE_HELP: &str = "Pipeline DSL:\n\
    tasker:task\n\
    tasker:task(args)        # shell may require quoting\n\
    tasker:task:arg          # recommended (no parentheses)\n\
\n\
Examples:\n\
    node-mutate:gen rustc:fuzz dump:pretty\n\
    load:gen llm:mask llm:infill rustc:fuzz gate:filter:query-stack+panic-func gate:filter:ice+success dump:raw\n\
\n\
Built-in stage args:\n\
    filter:query-stack+panic-func    # select ICE dedup filters\n\
    filter:ice+success+compile-error+hang  # select output result kinds\n\
    filter:query-stack+panic-func+ice+success  # mix is supported in one stage\n\
    dump:raw | dump:pretty\n\
\n\
rustc toolchain selection:\n\
    rustc stage can select a rustup toolchain via an arg:\n\
        rustc:fuzz:nightly-x86_64-unknown-linux-gnu\n\
        rustc:fuzz(stable-x86_64-unknown-linux-gnu)\n\
        rustc:fuzz:stage1-x86_64-unknown-linux-gnu   # custom toolchain via rustup toolchain link\n\
    This invokes: rustc +<toolchain> <args...>\n\
\n\
Shell note:\n\
    In zsh/bash, characters like '(', ')', '|' and '&' have special meaning.\n\
    Prefer the colon-arg form (tasker:task:arg) and '+' separators to avoid quoting.\n";

#[pyclass]
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, ValueEnum)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(ValueEnum, Clone, Debug)]
pub enum FuzzerType {
    Dummy,
    Rustc,
    Load,
    Splicer,
    Syn,
    NodeMutate,
    CppMutate,
    LLAMA,
    LLM,
    NoOp,
}
impl FuzzerType {
    pub fn new_fuzzer(&self, args: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        match self {
            FuzzerType::Dummy => Ok(crate::fuzz::fuzzbase::NoopFuzzer::new(args)?),
            FuzzerType::Rustc => Ok(crate::fuzz::fuzzbase::RustcFuzzer::new(args)?),
            FuzzerType::Load => Ok(crate::fuzz::fuzzbase::LoadFuzzer::new(args)?),
            FuzzerType::Splicer => Ok(crate::strategies::splicer::SplicerFuzzer::new(args)?),
            FuzzerType::Syn => {
                #[cfg(feature = "nightly-syn")]
                {
                    Ok(crate::strategies::syn::SynFuzzer::new(args)?)
                }
                #[cfg(not(feature = "nightly-syn"))]
                {
                    Err("FuzzerType::Syn 需要启用 Cargo feature `nightly-syn`，并在 nightly Rust 下构建".into())
                }
            }
            FuzzerType::NodeMutate => Ok(crate::strategies::synmutate::NodeMutater::new(args)?),
            FuzzerType::CppMutate => Ok(crate::strategies::cppmutate::CppMutater::new(args)?),
            FuzzerType::LLAMA => Ok(crate::strategies::models::llama_cpp::LlamaCppFuzzer::new(
                args,
            )?),
            FuzzerType::LLM => Ok(crate::strategies::models::python_api::LLMModule::new(args)?),
            FuzzerType::NoOp => Err("NoOp should not be parsed.".into()),
        }
    }
}




#[pyclass]
#[derive(Parser, Debug, Clone)]
#[command(after_long_help = PIPELINE_HELP)]
pub struct Args {
    #[arg(long, value_name = "FILE", help = "Load runtime config JSON")]
    #[pyo3(get)]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_name = "DIR", help = "Input corpus directory; can be repeated")]
    #[pyo3(get)]
    pub input: Vec<PathBuf>,

    #[arg(short, long, value_name = "DIR", help = "Output directory (existing files can be overwritten)")]
    #[arg(default_value = "out")]
    #[pyo3(get)]
    pub output: PathBuf,

    #[arg(short, long, value_name = "DIR", help = "Data directory for persistent filter state")]
    #[arg(default_value = ".")]
    #[pyo3(get)]
    pub datas: PathBuf,

    #[arg(short = 'j', value_name = "THREAD", help = "Worker thread count")]
    #[arg(default_value = "1")]
    #[pyo3(get)]
    pub nthread: usize,

    #[arg(long, help = "Skip hang-on-compile cases")]
    #[arg(default_value = "true")]
    #[pyo3(get)]
    pub skip_hang: bool,

    #[arg(long, help = "Enable unstable rustc flags path")]
    #[arg(default_value = "false")]
    #[pyo3(get)]
    pub use_unstable: bool,

    #[arg(long = "rustc-arg", value_name = "ARG", help = "Append raw rustc argument; can be repeated")]
    #[pyo3(get)]
    pub rustc_args: Vec<String>,

    #[arg(short, long, value_name = "FILE/NAME")]
    #[arg(default_value = "model.gguf")]
    #[pyo3(get)]
    pub model: String,

    #[arg(long)]
    #[arg(default_value = "false")]
    #[pyo3(get)]
    pub gpu: bool,

    #[arg(long = "mj", value_name = "THREAD")]
    #[arg(default_value = "10")]
    #[pyo3(get)]
    pub model_nthread: usize,

    #[arg(long = "log", value_name = "LEVEL", help = "Log verbosity")]
    #[arg(default_value = "info")]
    #[pyo3(get)]
    pub log_level: LogLevel,

    #[arg(short, long, value_name = "COUNT", help = "Target number of dumped outputs before stopping")]
    #[arg(default_value = "1")]
    #[pyo3(get)]
    pub _loopcnt: usize,

    #[arg(long, value_name = "COUNT", help = "Maximum pipeline iterations safeguard (optional)")]
    #[pyo3(get)]
    pub max_iter: Option<usize>,

    #[arg(long, value_name = "SECONDS", help = "Timeout in seconds")]
    #[pyo3(get)]
    pub timeout_sec: Option<u64>,

    #[arg(long, default_value_t = 0.1)]
    #[pyo3(get)]
    pub mutate_p: f64,

    #[arg(long, default_value_t = 30)]
    #[pyo3(get)]
    pub max_nested: usize,

    #[arg(long, default_value_t = 200)]
    #[pyo3(get)]
    pub max_analyze_depth: usize,

    #[arg(long, default_value_t = 1.05)]
    #[pyo3(get)]
    pub new_ice_adj_rate: f64,

    #[arg(long, default_value_t = 0.95)]
    #[pyo3(get)]
    pub dup_ice_adj_rate: f64,

    #[arg(long, default_value_t = 0.98)]
    #[pyo3(get)]
    pub choose_adj_rate: f64,

    #[arg(long, default_value_t = 0.5)]
    #[pyo3(get)]
    pub min_choose: f64,

    // Positional arguments

    // Define the jobs used in the program
    // given as a list of (FuzzerType, JobType)
    #[arg(value_name = "PIPELINE", num_args = 1..)]
    #[arg(value_parser = ValueParser::new(FuzzerJob::parser))]
    #[arg(help = "Pipeline stages (space-separated). See 'Pipeline DSL' section in --help.")]
    #[arg(default_value = "node-mutate:gen rustc:fuzz dump:pretty")]
    pub jobs: Vec<FuzzerJob>,
}

impl Args {
    pub fn synmutate_params(&self) -> SynMutateParams {
        SynMutateParams {
            mutate_p: self.mutate_p,
            max_nested: self.max_nested,
            max_analyze_depth: self.max_analyze_depth,
            new_ice_adj_rate: self.new_ice_adj_rate,
            dup_ice_adj_rate: self.dup_ice_adj_rate,
            choose_adj_rate: self.choose_adj_rate,
            min_choose: self.min_choose,
        }
    }

    pub fn apply_config_if_needed(&mut self) -> Result<(), Box<dyn Error>> {
        let p = match &self.config {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        self.apply_json_config(&p)
    }

    fn apply_json_config(&mut self, p: &Path) -> Result<(), Box<dyn Error>> {
        let txt = fs::read_to_string(p)?;
        let root: Value = serde_json::from_str(&txt)?;

        fn get_bool(v: &Value, key: &str) -> Option<bool> {
            v.get(key).and_then(Value::as_bool)
        }
        fn get_usize(v: &Value, key: &str) -> Option<usize> {
            v.get(key).and_then(Value::as_u64).map(|x| x as usize)
        }
        fn get_f64(v: &Value, key: &str) -> Option<f64> {
            v.get(key).and_then(Value::as_f64)
        }

        if let Some(skip_hang) = get_bool(&root, "skip_hang") {
            self.skip_hang = skip_hang;
        }
        if let Some(use_unstable) = get_bool(&root, "use_unstable") {
            self.use_unstable = use_unstable;
        }

        if let Some(v) = root.get("nthread").and_then(Value::as_u64) {
            self.nthread = v as usize;
        }
        if let Some(v) = root.get("loopcnt").and_then(Value::as_u64) {
            self._loopcnt = v as usize;
        }
        if let Some(v) = root.get("max_iter").and_then(Value::as_u64) {
            self.max_iter = Some(v as usize);
        }
        if let Some(v) = root.get("timeout_sec").and_then(Value::as_u64) {
            self.timeout_sec = Some(v);
        }

        if let Some(v) = root.get("input").and_then(Value::as_array) {
            self.input = v
                .iter()
                .filter_map(Value::as_str)
                .map(PathBuf::from)
                .collect();
        }
        if let Some(v) = root.get("output").and_then(Value::as_str) {
            self.output = PathBuf::from(v);
        }
        if let Some(v) = root.get("datas").and_then(Value::as_str) {
            self.datas = PathBuf::from(v);
        }

        if let Some(v) = root.get("rustc_args").and_then(Value::as_array) {
            self.rustc_args = v
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }

        if let Some(v) = root.get("jobs").and_then(Value::as_array) {
            let mut jobs = Vec::new();
            for s in v.iter().filter_map(Value::as_str) {
                jobs.push(FuzzerJob::parser(s).map_err(|e| e.to_string())?);
            }
            if !jobs.is_empty() {
                self.jobs = jobs;
            }
        }

        if let Some(syn) = root.get("synmutate") {
            if let Some(x) = get_f64(syn, "mutate_p") {
                self.mutate_p = x;
            }
            if let Some(x) = get_usize(syn, "max_nested") {
                self.max_nested = x;
            }
            if let Some(x) = get_usize(syn, "max_analyze_depth") {
                self.max_analyze_depth = x;
            }
            if let Some(x) = get_f64(syn, "new_ice_adj_rate") {
                self.new_ice_adj_rate = x;
            }
            if let Some(x) = get_f64(syn, "dup_ice_adj_rate") {
                self.dup_ice_adj_rate = x;
            }
            if let Some(x) = get_f64(syn, "choose_adj_rate") {
                self.choose_adj_rate = x;
            }
            if let Some(x) = get_f64(syn, "min_choose") {
                self.min_choose = x;
            }
        }

        Ok(())
    }
}

static LOG_LEVEL: LazyLock<RwLock<LogLevel>> = LazyLock::new(|| RwLock::new(LogLevel::Warn));

pub fn get_log_level() -> LogLevel {
    let l = LOG_LEVEL.read().unwrap();
    *l
}

pub fn set_log_level(s: &LogLevel) {
    let mut _l = LOG_LEVEL.write().unwrap();
    *_l = *s;
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Debug {
            use colored::Colorize;
            println!("{} {}", "[DEBUG]".blue(), format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Info {
            // println!($($arg)*);
            use colored::Colorize;
            println!("{} {}", "[INFO]".green(), format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Warn {
            // eprintln!($($arg)*);
            use colored::Colorize;
            eprintln!("{} {}", "[WARN]".yellow(), format!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Error {
            // eprintln!($($arg)*);
            use colored::Colorize;
            println!("{} {}", "[ERROR]".red(), format!($($arg)*));
        }
    }
}
