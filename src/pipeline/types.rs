use clap::ValueEnum;
use std::{error::Error, fmt::Display};

use crate::{conf::Args, ice_process};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobType {
    Gen,
    Mask,
    Infill,
    Fuzz,
    Dump,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinStage {
    Filter,
    Dump,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageKind {
    Builtin(BuiltinStage),
    Job(JobType),
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum FilterJob {
    Dummy,
    QueryStack,
    PanicFunc,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultKind {
    Ice,
    Hang,
    Success,
    CompileError,
}

#[derive(Clone, Debug)]
pub struct SynMutateParams {
    pub mutate_p: f64,
    pub max_nested: usize,
    pub max_analyze_depth: usize,
    pub new_ice_adj_rate: f64,
    pub dup_ice_adj_rate: f64,
    pub choose_adj_rate: f64,
    pub min_choose: f64,
}

impl Display for FilterJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterJob::Dummy => write!(f, "dummy"),
            FilterJob::QueryStack => write!(f, "query-stack"),
            FilterJob::PanicFunc => write!(f, "panic-func"),
        }
    }
}

impl FilterJob {
    pub fn new_filter(&self, _: &Args) -> Result<Box<dyn ice_process::ICEFilter>, Box<dyn Error>> {
        match self {
            FilterJob::Dummy => Ok(crate::ice_process::DummyFilter::new()),
            FilterJob::QueryStack => Ok(crate::ice_process::querystack::QueryStackFilter::new()),
            FilterJob::PanicFunc => Ok(crate::ice_process::panicfunc::PanicFuncFilter::new()),
        }
    }
}
