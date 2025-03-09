use clap::{Parser, arg};
use std::{path::PathBuf, sync::LazyLock};

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value = "out.rs")]
    pub output: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    #[arg(default_value = "out.bin")]
    pub binary: PathBuf,

    #[arg(short, long, value_name = "LEVEL")]
    #[arg(default_value = "info")]
    pub log_level: String,
}

impl Args {
    pub fn get_log_level(&self) -> LogLevel {
        match self.log_level.as_str() {
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
}

pub static ARGS: LazyLock<Args> = LazyLock::new(
    || Args::parse(),
);

pub fn args() -> &'static Args {
    &ARGS
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::conf::args().get_log_level() >= $crate::conf::LogLevel::Debug {
            eprintln!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::conf::args().get_log_level() >= $crate::conf::LogLevel::Info {
            eprintln!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::conf::args().get_log_level() >= $crate::conf::LogLevel::Warn {
            eprintln!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::conf::args().get_log_level() >= $crate::conf::LogLevel::Error {
            eprintln!($($arg)*);
        }
    }
}

