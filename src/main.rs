// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// #![feature(trace_macros)]
// trace_macros!(true);

use std::error::Error;

use clap::Parser;
use conf::{Args, set_log_level};

mod conf;
mod fuzz;
mod ice_process;
mod util;

mod pipeline;

mod strategies;
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    args.apply_config_if_needed()?;
    let args = Box::leak(args.into());
    set_log_level(&args.log_level);

    debug!("{:#?}", args);

    pipeline::runtime::execute(args)
}
