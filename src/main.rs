#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(concat_idents)]
#![feature(box_into_inner)]
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
// #![feature(trace_macros)]
// trace_macros!(true);

use std::{
    env::temp_dir,
    error::Error,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use clap::Parser;
use conf::{Args, set_log_leven};
use fuzz::{
    feature_list::FEATURES,
    fuzzbase::{DummyFuzzer, FResult, Fuzzer},
};
use ice_process::{
    DummyFilter, ICEFilter, flagbisect::filter_flags, panicfunc::PanicFuncFilter,
    querystack::QueryStackFilter,
};
// use strategies::models::ModelFuzzer;
#[allow(unused_imports)]
use strategies::{splicer::SplicerFuzzer, syn::SynFuzzer};
use util::gen_alnum;

mod conf;
mod fuzz;
mod ice_process;
mod util;

mod strategies;

const THREAD_SET_CNT: usize = 3;

static EXTRA_ARGS: [&str; 9] = [
    "-C",
    "opt-level=3",
    "--edition",
    "2024",
    "--emit=mir",
    "-Zno-codegen",
    "-Zmir-opt-level=4",
    "-Zvalidate-mir",
    "--crate-type=lib",
];

fn run<T: Fuzzer>(
    args: &Args,
    fuzzer: Arc<Mutex<Box<dyn Fuzzer>>>,
    idx: Arc<AtomicUsize>,
    filters: Vec<Arc<Mutex<Box<dyn ICEFilter>>>>,
) -> Result<(), Box<dyn Error>> {
    let tmp_source = temp_dir().join(format!("nfuzz_{}.rs", gen_alnum(4)));
    let tmp_bin = temp_dir().join(format!("nfuzz_{}.bin", gen_alnum(4)));
    loop {
        let cidx = idx.fetch_add(1, Ordering::SeqCst);
        if !args._infloop && cidx >= args._loopcnt {
            break;
        }

        let code = {
            let mut lock = fuzzer.lock().map_err(|e| e.to_string())?;
            lock.generate()?
        };
        let compile_res = T::compile(&code, &tmp_source, &tmp_bin, &EXTRA_ARGS)?;
        println!("{cidx} - CRES: {}", compile_res.1);

        if args.force_dump {
            let p = args.output.join(format!("gen_{cidx}.rs"));
            println!(
                "\t\tDump to: {}",
                p.as_os_str().to_str().unwrap_or_default()
            );
            T::dump(&code, &p)?;
            let mut addon_f = OpenOptions::new().append(true).open(&p)?;
            addon_f.write_fmt(format_args!(
                "\n\n// Compile Args: {}",
                EXTRA_ARGS.join(" ")
            ))?;
            addon_f.write_fmt(format_args!(
                "\n\n// Compile Flags: {}",
                FEATURES.join(" ")
            ))?;
            continue;
        }

        if !matches!(compile_res.1, FResult::InternalCompileError(..)) {
            continue;
        }

        let mut filter_pass = 0;
        for filter in &filters {
            let mut lock = filter.lock().map_err(|s| s.to_string())?;
            if !lock.filter(&compile_res.1) {
                filter_pass += 1;
                let _ = lock.add(&compile_res.1);
            }
        }

        if filter_pass == 0 {
            println!("\t\tICE already exists, filtering out");
            continue;
        }

        let p = args.output.join(format!("gen_{cidx}.rs"));
        println!(
            "\t\tDump to: {}",
            p.as_os_str().to_str().unwrap_or_default()
        );
        T::dump(&code, &p)?;
        let mut addon_f = OpenOptions::new().append(true).open(&p)?;
        let flags = FEATURES.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let minimized_flag = filter_flags::<T>(flags, &code, &tmp_source, &tmp_bin, &EXTRA_ARGS)?;
        addon_f.write_fmt(format_args!(
            "\n\n// Compile Args: {}",
            EXTRA_ARGS.join(" ")
        ))?;
        addon_f.write_fmt(format_args!(
            "\n\n// Original Flags: {}",
            FEATURES.join(" ")
        ))?;
        addon_f.write_fmt(format_args!(
            "\n\n// Compile Flags: {}",
            minimized_flag.join(" ")
        ))?;
    }
    info!("Work thread exited!");
    Ok(())
}

// type FuzzerType = DummyFuzzer;
// type FuzzerType = SplicerFuzzer;
type FuzzerType = SynFuzzer;
// type FuzzerType = ModelFuzzer;
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let args = Box::leak(args.into());
    set_log_leven(&args.log_level);

    debug!("{:#?}", args);

    let fuzzer = FuzzerType::new(&args)?;

    let filters: Vec<_> = vec![
        QueryStackFilter::new(),
        PanicFuncFilter::new(),
        DummyFilter::new(),
    ]
    .into_iter()
    .collect();

    create_dir_all(&args.output)?;
    info!("Begin generating...");

    let idx: Arc<AtomicUsize> = Arc::new(0.into());

    if args.nthread <= 1 {
        let fuzzer = Arc::new(Mutex::new(fuzzer));
        let filters: Vec<_> = filters
            .iter()
            .map(|x| Arc::new(Mutex::new(dyn_clone::clone_box(&**x))))
            .collect();
        run::<FuzzerType>(args, fuzzer, idx.clone(), filters)?;
    } else {
        let mut handles = Vec::default();
        let barrier = Arc::new(Barrier::new(args.nthread));
        let mut i = 0;

        let args = &*args;
        info!(
            "Due to multi-thread needs to copy some data, the spawn procedure could be as slow as the init process :("
        );
        info!("Please wait patiently~ :P");

        let filters: Vec<_> = filters
            .iter()
            .map(|x| Arc::new(Mutex::new(dyn_clone::clone_box(&**x))))
            .collect();
        while i < args.nthread {
            let fuzzer = dyn_clone::clone_box(&*fuzzer);
            let fuzzer = Arc::new(Mutex::new(fuzzer));

            for _ in 0..THREAD_SET_CNT {
                if i >= args.nthread {
                    break;
                }
                let fuzzer = fuzzer.clone();
                let idx = idx.clone();
                let filters = filters.clone();
                let barrier = barrier.clone();
                let handle = thread::spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                    barrier.wait();
                    run::<FuzzerType>(args, fuzzer, idx, filters).map_err(|e| e.to_string().into())
                });
                handles.push(handle);
                info!("Thread {i} spawned~");
                i += 1;
            }
        }
        info!("Begin to execute w~");
        while let Some(handle) = handles.pop() {
            let res = handle.join().unwrap();
            res.map_err(|e| e as Box<dyn Error>)?;
        }
    }

    Ok(())
}
