#![feature(rustc_private)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(concat_idents)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(box_into_inner)]
#![feature(trace_macros)]
// trace_macros!(true);

mod conf;
mod fuzz;
mod strategy;
mod util;

use std::error::Error;

use clap::Parser;
use conf::Args;
use fuzz::fuzzbase::FResult;
// #[allow(unused_imports)]
// use fuzz::rustcfuzz::RustCFuzzer;
#[allow(unused_imports)]
use fuzz::synfuzz::SynFuzzer;

fn main() -> Result<(), Box<dyn Error>> {
    let extra_compile_args = vec![
        "-C".to_string(),
        "opt-level=3".to_string(),
        "--edition".to_string(),
        "2024".to_string(),
        "--emit=mir".to_string(),
        "-Zmir-opt-level=4".to_string(),
        "-Zvalidate-mir".to_string(),
    ];

    let args = Args::parse();

    let mut fuzzer = SynFuzzer::new(&args.input, &extra_compile_args)?;
    // let mut fuzzer = RustCFuzzer::new(&args.input, &extra_compile_args)?;
    // fuzzer.replace()?;

    let mut idx = 0;
    loop {
        idx += 1;
        fuzzer.generate()?;
        fuzzer.dump(&args.output)?;

        let compile_res = fuzzer.compile(&args.binary, &extra_compile_args)?;
        println!("{} - CRES: {}", idx, compile_res);

        if let FResult::CompileSuccess(_) = compile_res {
            let run_res = fuzzer.run(&args.binary)?;
            println!("Run return with {}", run_res.status);
            println!("StdOut:");
            print!("{}\n\n", String::from_utf8(run_res.stdout)?);
            println!("<StdOut end>");
            println!("StdErr:");
            print!("{}\n\n", String::from_utf8(run_res.stderr)?);
            println!("<StdErr end>");
        }

        if !args._loop {
            break;
        }

        if !matches!(compile_res, FResult::CompileError(..)) {
            break;
        }
    }

    Ok(())
}
