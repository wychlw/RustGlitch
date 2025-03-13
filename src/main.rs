#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(concat_idents)]
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// #![feature(box_into_inner)]
// #![feature(trace_macros)]
// trace_macros!(true);

use std::{
    env::temp_dir,
    error::Error,
    fs::create_dir_all,
    sync::{
        Arc, RwLock,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use clap::Parser;
use conf::{Args, set_log_leven};
use fuzz::{
    fuzzbase::{FResult, Fuzzer},
    splicer::SplicerFuzzer,
};
use util::gen_alnum;

mod conf;
mod fuzz;
mod ice_process;
mod util;

static EXTRA_ARGS: [&str; 8] = [
    "-C",
    "opt-level=3",
    "--edition",
    "2024",
    "--emit=mir",
    "-Zno-codegen",
    "-Zmir-opt-level=4",
    "-Zvalidate-mir",
];

fn run<T: Fuzzer>(
    args: &Args,
    fuzzer: Arc<RwLock<Box<dyn Fuzzer>>>,
    idx: Arc<AtomicUsize>,
) -> Result<(), Box<dyn Error>> {
    let tmp_source = temp_dir().join(format!("nfuzz_{}.rs", gen_alnum(4)));
    let tmp_bin = temp_dir().join(format!("nfuzz_{}.rs", gen_alnum(4)));
    loop {
        let cidx = idx.fetch_add(1, Ordering::SeqCst);
        if !args._infloop && cidx >= args._loopcnt {
            break;
        }

        let code = {
            let mut lock = fuzzer.write().map_err(|e| e.to_string())?;
            lock.generate()?
        };
        let compile_res = T::compile(&code, &tmp_source, &tmp_bin, &EXTRA_ARGS)?;
        println!("{cidx} - CRES: {compile_res}");
        if args.force_dump || matches!(compile_res, FResult::InternalCompileError(..)) {
            let p = args.output.join(format!("gen_{cidx}.rs"));
            println!(
                "\t\tDump to: {}",
                p.as_os_str().to_str().unwrap_or_default()
            );
            T::dump(&code, &p)?;
        }
    }
    Ok(())
}

type FuzzerType = SplicerFuzzer;
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let args = Box::leak(args.into());
    set_log_leven(&args.log_level);

    debug!("{:#?}", args);

    let fuzzer = FuzzerType::new(&args.input)?;
    let fuzzer = Arc::new(RwLock::new(fuzzer));

    create_dir_all(&args.output)?;
    info!("Begin generating...");

    let idx: Arc<AtomicUsize> = Arc::new(0.into());

    if args.nthread <= 1 {
        run::<FuzzerType>(args, fuzzer.clone(), idx.clone())?;
    } else {
        let mut handles = Vec::default();
        for _ in 0..args.nthread {
            let args = &*args;
            let fuzzer = fuzzer.clone();
            let idx = idx.clone();
            let handle = thread::spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                run::<FuzzerType>(args, fuzzer, idx).map_err(|e| e.to_string().into())
            });
            handles.push(handle);
        }
        while !handles.is_empty() {
            let handle = handles.pop().unwrap();
            let res = handle.join().unwrap();
            res.map_err(|e| e as Box<dyn Error>)?;
        }
    }

    Ok(())
}
