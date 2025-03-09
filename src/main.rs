#![feature(rustc_private)]
#![feature(macro_metavar_expr_concat)]

mod conf;
mod fuzz;
mod util;
mod display;
mod stragety;

use std::error::Error;

use conf::args;
#[allow(unused_imports)]
// use fuzz::rustcfuzz::RustCFuzzer;
#[allow(unused_imports)]
use fuzz::synfuzz::SynFuzzer;

fn main() -> Result<(), Box<dyn Error>> {

    let extra_compile_args = vec![
        "-C".to_string(),
        "opt-level=3".to_string(),
        "--edition".to_string(),
        "2024".to_string(),
    ];

    let mut fuzzer = SynFuzzer::new(&args().input, &extra_compile_args)?;
    // let mut fuzzer = RustCFuzzer::new(&args.input, &extra_compile_args)?;
    fuzzer.replace()?;
    fuzzer.dump(&args().output)?;

    let compile_res = fuzzer.compile(&args().binary, &extra_compile_args)?;
    println!("CRES: {:?}", compile_res);
    Ok(())
}
