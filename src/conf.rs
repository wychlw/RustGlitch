use clap::{arg, Parser};
use std::{
    path::PathBuf, sync::{LazyLock, RwLock}
};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_name = "DIR")]
    #[arg(default_value = "in")]
    pub input: Vec<PathBuf>,

    #[arg(short, long, value_name = "DIR")]
    #[arg(default_value = "out")]
    pub output: PathBuf,

    #[arg(short, long, value_name = "DIR")]
    #[arg(default_value = ".")]
    pub datas: PathBuf,

    #[arg(short='j', value_name = "THREAD")]
    #[arg(default_value = "1")]
    pub nthread: usize,

    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value = "model.gguf")]
    pub model: PathBuf,

    #[arg(long)]
    #[arg(default_value = "false")]
    pub gpu: bool,

    #[arg(long="mj", value_name = "THREAD")]
    #[arg(default_value = "10")]
    pub model_nthread: usize,


    #[arg(long="log", value_name = "LEVEL")]
    #[arg(default_value = "info")]
    pub log_level: String,

    #[arg(short, long, value_name = "LOOP")]
    #[arg(default_value = "1")]
    pub _loopcnt: usize,
    
    #[arg(long="inf", value_name = "LOOP", conflicts_with = "_loopcnt")]
    #[arg(default_value = "false")]
    pub _infloop: bool,

    #[arg(long, value_name = "FORCE_DUMP")]
    #[arg(default_value = "false")]
    pub force_dump: bool,
}

static LOG_LEVEL: LazyLock<RwLock<LogLevel>> = LazyLock::new(|| RwLock::new(LogLevel::Info));

pub fn get_log_level() -> LogLevel {
    let l = LOG_LEVEL.read().unwrap();
    *l
}

pub fn set_log_level(s: &str) {
    let l = match s {
        "error" => LogLevel::Error,
        "warn" => LogLevel::Warn,
        "info" => LogLevel::Info,
        "debug" => LogLevel::Debug,
        _ => panic!("No such level")
    };
    let mut _l = LOG_LEVEL.write().unwrap();
    *_l = l;
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Debug {
            println!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Info {
            println!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Warn {
            eprintln!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::conf::get_log_level() >= $crate::conf::LogLevel::Error {
            eprintln!($($arg)*);
        }
    }
}
