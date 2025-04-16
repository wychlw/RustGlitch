use std::{
    env::temp_dir, error::Error, ffi::OsStr, fmt::Display, fs, path::Path, process::{Command, Output, Stdio}
};

use dyn_clone::DynClone;
use regex::Regex;
use walkdir::WalkDir;

use crate::{conf::Args, debug, fuzz::feature_list::FEATURES};

#[derive(Debug)]
pub enum FResult {
    CompileSuccess(Output),
    CompileError(Output),
    InternalCompileError(Output),
    RunSuccess,
    RunError,
}
impl Display for FResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileSuccess(_) => write!(f, "Compile Success :)"),
            Self::CompileError(_) => write!(f, "Compile Error :|"),
            Self::InternalCompileError(_) => write!(f, "ICE?!?!?! :("),
            _ => write!(f, "Not defined"),
        }
    }
}
pub trait Fuzzer: Send + Sync + DynClone {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn dump(code: &[u8], output: &Path) -> Result<(), Box<dyn Error>>
    where
        Self: Sized,
    {
        fuzzer_dump(code, output)
    }
    fn compile(
        code: &[u8],
        output_source: &Path,
        output_bin: &Path,
        extra_args: &[&str],
    ) -> Result<(Vec<String>, FResult), Box<dyn Error>>
    where
        Self: Sized,
    {
        Self::compile_with_features(code, output_source, output_bin, extra_args, &FEATURES)
    }
    fn compile_with_features(
        code: &[u8],
        output_source: &Path,
        output_bin: &Path,
        extra_args: &[&str],
        features: &[&str],
    ) -> Result<(Vec<String>, FResult), Box<dyn Error>>
    where
        Self: Sized,
    {
        fuzzer_compile::<Self>(code, output_source, output_bin, extra_args, features)
    }
    // fn run(&mut self, bin_path: &Path) -> Result<Output, Box<dyn Error>> {
    //     let mut cmd = Command::new(bin_path);
    //     let status = cmd.output()?;
    //     Ok(status)
    // }
}

#[derive(Default, Clone)]
pub struct DummyFuzzer {
    pub codes: Vec<Vec<u8>>,
    pub idx: usize,
}
impl DummyFuzzer {
    pub fn new(conf: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let dirs = &conf.input;
        let mut codes = vec![];
        for dir in dirs {
            for f in WalkDir::new(dir) {
                let entry = f?;
                if !entry.file_type().is_file() {
                    continue;
                }
                if entry.path().extension() != Some(OsStr::new("rs")) {
                    continue;
                }
                let code = fs::read(entry.path())?;
                codes.push(code);
            }
        }
        let res = Self { codes, idx: 0 };
        let res = Box::new(res);
        Ok(res)
    }
}
impl Fuzzer for DummyFuzzer {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.idx >= self.codes.len() {
            self.idx = 0;
        }
        let code = self.codes[self.idx].clone();
        self.idx += 1;
        Ok(code)
    }
}

pub fn code_mask_feature(code: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    const RE: &str = r"^(#!\[feature\([a-zA-Z0-9_,]*\)\])$";
    const RE_REP: &str = r"\\$1";
    let re = Regex::new(RE)?;
    let res = re
        .replace_all(str::from_utf8(code)?, RE_REP)
        .as_bytes()
        .to_owned();
    Ok(res)
}

pub fn fuzzer_compile<T: Fuzzer>(
    code: &[u8],
    output_source: &Path,
    output_bin: &Path,
    extra_args: &[&str],
    features: &[&str],
) -> Result<(Vec<String>, FResult), Box<dyn Error>> {
    // (Args, Result)
    {
        let tmp_file = temp_dir().join(output_source);
        T::dump(code, &tmp_file)?;
        let args = [
            tmp_file.to_str().unwrap().to_string(),
            "-o".to_string(),
            output_bin.to_str().unwrap().to_string(),
        ];
        let args: Vec<String> = args
            .into_iter()
            .chain(extra_args.iter().map(|s| s.to_string()))
            .collect();
        let extra_args: Vec<String> = features
            .iter()
            .map(|s| format!("-Zcrate-attr=feature({s})"))
            .collect();
        let args: Vec<String> = args.into_iter().chain(extra_args).collect();
        let mut cmd = Command::new("rustc");
        cmd.env("RUST_BACKTRACE", "1");
        cmd.args(args.clone());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        let status = cmd.output()?;
        debug!("Code: {}", status.status.code().unwrap());
        if status.status.success() {
            return Ok((args, FResult::CompileSuccess(status)));
        }
        if status.status.code().unwrap() == 1 {
            return Ok((args, FResult::CompileError(status)));
        }
        Ok((args, FResult::InternalCompileError(status)))
    }
}
pub fn fuzzer_dump(code: &[u8], output: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(output, code)?;
    Ok(())
}
