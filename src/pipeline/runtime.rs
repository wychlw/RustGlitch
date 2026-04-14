use std::{
    env::temp_dir,
    error::Error,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::PathBuf,
    sync::{
        Arc, Barrier, LazyLock, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Instant,
};

use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

use clap::ValueEnum;

use crate::{error, info, warn};

use crate::{
    conf::Args,
    fuzz::{
        feature_list::FEATURES,
        fuzzbase::{FResult, Fuzzer, NoopFuzzer, RustcFuzzer, fuzzer_compile_with_toolchain},
    },
    ice_process::{ICEFilter, flagbisect::{filter_flags, filter_flags_with_toolchain}},
    pipeline::{BuiltinStage, FilterJob, FuzzerJob, JobType, ResultKind, StageKind},
    util::gen_alnum,
};

const THREAD_SET_CNT: usize = 3;
const STACK_SIZE: usize = 1024 * 1024 * 128; // 128MB

static EXTRA_ARGS_BASE: [&str; 6] = [
    "-C",
    "opt-level=3",
    "--edition",
    "2024",
    "--crate-type=lib",
    "--emit=mir",
];

static STOP: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));
static DEFAULT_FILTERS: [FilterJob; 2] = [FilterJob::QueryStack, FilterJob::PanicFunc];
static DEFAULT_KEEP: [ResultKind; 4] = [
    ResultKind::Ice,
    ResultKind::Success,
    ResultKind::CompileError,
    ResultKind::Hang,
];

type SharedFilter = Arc<Mutex<Box<dyn ICEFilter>>>;

struct RunControl {
    started_at: Instant,
    ice: AtomicUsize,
    success: AtomicUsize,
    compile_error: AtomicUsize,
    dumped: AtomicUsize,
    iter: AtomicUsize,
}
impl RunControl {
    fn new() -> Self {
        Self {
            started_at: Instant::now(),
            ice: AtomicUsize::new(0),
            success: AtomicUsize::new(0),
            compile_error: AtomicUsize::new(0),
            dumped: AtomicUsize::new(0),
            iter: AtomicUsize::new(0),
        }
    }
    fn inc_result(&self, kind: ResultKind) {
        match kind {
            ResultKind::Ice => {
                self.ice.fetch_add(1, Ordering::Relaxed);
            }
            ResultKind::Success => {
                self.success.fetch_add(1, Ordering::Relaxed);
            }
            ResultKind::CompileError => {
                self.compile_error.fetch_add(1, Ordering::Relaxed);
            }
            ResultKind::Hang => {}
        }
        self.dumped.fetch_add(1, Ordering::Relaxed);
    }
    fn inc_iter(&self) {
        self.iter.fetch_add(1, Ordering::Relaxed);
    }
    fn snapshot(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.ice.load(Ordering::Relaxed),
            self.success.load(Ordering::Relaxed),
            self.compile_error.load(Ordering::Relaxed),
            self.dumped.load(Ordering::Relaxed),
            self.iter.load(Ordering::Relaxed),
        )
    }
    fn output_target_reached(&self, args: &Args) -> bool {
        self.dumped.load(Ordering::Relaxed) >= args._loopcnt
    }
    fn iter_limit_reached(&self, args: &Args) -> bool {
        let Some(max_iter) = args.max_iter else {
            return false;
        };
        self.iter.load(Ordering::Relaxed) >= max_iter
    }
    fn timeout_reached(&self, args: &Args) -> bool {
        let Some(timeout_sec) = args.timeout_sec else {
            return false;
        };
        self.started_at.elapsed().as_secs() >= timeout_sec
    }
    fn should_stop(&self, args: &Args) -> bool {
        self.output_target_reached(args) || self.iter_limit_reached(args) || self.timeout_reached(args)
    }
}

fn split_tokens(expr: &str) -> Vec<String> {
    expr.split(['|', '&', ',', '+'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn parse_filter_expr(expr: &str) -> Result<(Option<Vec<FilterJob>>, Option<Vec<ResultKind>>), String> {
    let mut filters = Vec::new();
    let mut keeps = Vec::new();

    for tk in split_tokens(expr) {
        if let Ok(v) = FilterJob::from_str(&tk, true) {
            filters.push(v);
            continue;
        }
        if let Ok(v) = ResultKind::from_str(&tk, true) {
            keeps.push(v);
            continue;
        }
        return Err(format!("Invalid filter token: {tk}"));
    }

    if filters.is_empty() && keeps.is_empty() {
        return Err("Invalid filter args".to_string());
    }

    let filters = if filters.is_empty() { None } else { Some(filters) };
    let keeps = if keeps.is_empty() { None } else { Some(keeps) };
    Ok((filters, keeps))
}

enum JobCB {
    Null,
    InformICE(bool), // bool : is dup
}

#[derive(Clone, Copy)]
enum DumpMode {
    Raw,
    Pretty,
}

#[derive(Clone)]
struct DumpPlan {
    kind: ResultKind,
    prefix: String,
    compile_args: Vec<String>,
    extra_comment: Option<String>,
    can_dump: bool,
}

struct PipelineState {
    code: Vec<Vec<u8>>,
    dump_plan: Option<DumpPlan>,
    dump_plan_written: usize,
    dropped: bool,
}
impl PipelineState {
    fn new() -> Self {
        Self {
            code: Vec::new(),
            dump_plan: None,
            dump_plan_written: 0,
            dropped: false,
        }
    }
}

#[derive(Clone)]
struct Job {
    fuzzer: Arc<Mutex<Box<dyn Fuzzer>>>,
    job: JobType,
    dump_mode: DumpMode,
    rustc_toolchain: Option<String>,
    tmp_source: PathBuf,
    tmp_bin: PathBuf,
}
impl Job {
    pub fn new(
        fuzzer: Box<dyn Fuzzer>,
        job: JobType,
        dump_mode: DumpMode,
        rustc_toolchain: Option<String>,
    ) -> Self {
        Self {
            fuzzer: Arc::new(Mutex::new(fuzzer)),
            job,
            dump_mode,
            rustc_toolchain,
            tmp_source: temp_dir().join(format!("nfuzz_{}.rs", gen_alnum(4))),
            tmp_bin: temp_dir().join(format!("nfuzz_{}.bin", gen_alnum(4))),
        }
    }
    pub fn deep_clone(&self) -> Self {
        let mut lock = self.fuzzer.lock().map_err(|e| e.to_string()).unwrap();
        let fuzzer = lock.as_mut();
        let fuzzer = dyn_clone::clone_box(&*fuzzer);
        let fuzzer = Arc::new(Mutex::new(fuzzer));
        Self {
            fuzzer,
            job: self.job,
            dump_mode: self.dump_mode,
            rustc_toolchain: self.rustc_toolchain.clone(),
            tmp_source: self.tmp_source.clone(),
            tmp_bin: self.tmp_bin.clone(),
        }
    }
}

impl Job {
    fn maybe_pretty_code(code: &[u8], dump_mode: DumpMode) -> Vec<u8> {
        if !matches!(dump_mode, DumpMode::Pretty) {
            return code.to_vec();
        }
        let Ok(s) = std::str::from_utf8(code) else {
            return code.to_vec();
        };
        let Ok(ast) = syn::parse_file(s) else {
            return code.to_vec();
        };
        prettyplease::unparse(&ast).into_bytes()
    }

    fn should_keep(_args: &Args, keep_override: Option<&[ResultKind]>, kind: ResultKind) -> bool {
        if let Some(v) = keep_override {
            v.contains(&kind)
        } else {
            DEFAULT_KEEP.contains(&kind)
        }
    }

    fn build_rustc_args(args: &Args) -> Vec<String> {
        let mut v: Vec<String> = EXTRA_ARGS_BASE.iter().map(|x| x.to_string()).collect();
        if args.use_unstable {
            v.extend([
                "-Zno-codegen".to_string(),
                "-Zmir-opt-level=4".to_string(),
                "-Zvalidate-mir".to_string(),
            ]);
        }
        v.extend(args.rustc_args.clone());
        v
    }

    fn dump_result(
        args: &Args,
        control: &RunControl,
        code: &[u8],
        idx: usize,
        plan: &DumpPlan,
    ) -> Result<(), Box<dyn Error>> {
        let p = args.output.join(format!("{}_{}.rs", plan.prefix, idx));
        info!("\t\tDump to: {}", p.as_os_str().to_str().unwrap_or_default());
        <NoopFuzzer as Fuzzer>::dump(code, &p)?;
        let mut addon_f = OpenOptions::new().append(true).open(&p)?;
        addon_f.write_fmt(format_args!("\n\n// Compile Args: {}", plan.compile_args.join(" ")))?;
        if args.use_unstable {
            addon_f.write_fmt(format_args!("\n\n// Original Flags: {}", FEATURES.join(" ")))?;
        }
        if let Some(s) = plan.extra_comment.as_deref() {
            addon_f.write_fmt(format_args!("\n\n// {}", s))?;
        }
        control.inc_result(plan.kind);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn do_once(
        &self,
        args: &Args,
        control: &RunControl,
        has_dump_job: bool,
        keep_override: Option<&[ResultKind]>,
        idx: usize,
        step_idx: usize,
        state: &mut PipelineState,
        filters: &mut Vec<SharedFilter>,
    ) -> Result<JobCB, Box<dyn Error>> {
        let mut lock = self.fuzzer.lock().map_err(|e| e.to_string())?;
        match self.job {
            JobType::Gen => {
                let new_code = lock.generate()?;
                state.code.push(new_code);
            }
            JobType::Mask => {
                let maskfuzzer = lock.as_mut().as_mask_fuzzer_mut()?;
                let last = state.code.pop().ok_or("No code to mask")?;
                let (code_prefix, code_suffix) = maskfuzzer.mask(&last)?;
                state.code.push(code_prefix);
                state.code.push(code_suffix);
            }
            JobType::Infill => {
                let infillfuzzer = lock.as_mut().as_infill_fuzzer_mut()?;
                let suffix = state.code.pop().ok_or("No code to infill")?;
                let prefix = state.code.pop().ok_or("No code to infill")?;
                let infill_code = infillfuzzer.infill(&prefix, &suffix)?;
                state.code.push(infill_code);
            }
            JobType::Fuzz => {
                if state.code.is_empty() {
                    Err("You can't fuzz without code")?;
                }
                let extra_owned = Self::build_rustc_args(args);
                let extra_ref: Vec<&str> = extra_owned.iter().map(String::as_str).collect();
                let mut compile_args_for_dump = extra_owned.clone();
                if let Some(toolchain) = self.rustc_toolchain.as_deref() {
                    compile_args_for_dump.insert(0, format!("+{toolchain}"));
                }
                let compile_res = fuzzer_compile_with_toolchain::<RustcFuzzer>(
                    self.rustc_toolchain.as_deref(),
                    &state.code[0],
                    &self.tmp_source,
                    &self.tmp_bin,
                    &extra_ref,
                    if args.use_unstable { &FEATURES } else { &[] },
                );
                let compile_res = match compile_res {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Read thread error...: {e}");
                        return Ok(JobCB::Null);
                    }
                };
                info!("{idx} - CRES: {}", compile_res.1);

                state.dump_plan = None;
                state.dump_plan_written = 0;
                state.dropped = false;

                if matches!(&compile_res.1, FResult::CompileSuccess(..)) {
                    if !Self::should_keep(args, keep_override, ResultKind::Success) {
                        state.dropped = true;
                        return Ok(JobCB::Null);
                    }
                    let plan = DumpPlan {
                        kind: ResultKind::Success,
                        prefix: "success".to_string(),
                        compile_args: compile_args_for_dump,
                        extra_comment: None,
                        can_dump: true,
                    };
                    if has_dump_job {
                        state.dump_plan = Some(plan);
                    } else {
                        Self::dump_result(args, control, &state.code[0], idx, &plan)?;
                    }
                    return Ok(JobCB::Null);
                }

                if matches!(&compile_res.1, FResult::CompileError(..)) {
                    if !Self::should_keep(args, keep_override, ResultKind::CompileError) {
                        state.dropped = true;
                        return Ok(JobCB::Null);
                    }
                    let plan = DumpPlan {
                        kind: ResultKind::CompileError,
                        prefix: "compile_error".to_string(),
                        compile_args: compile_args_for_dump,
                        extra_comment: None,
                        can_dump: true,
                    };
                    if has_dump_job {
                        state.dump_plan = Some(plan);
                    } else {
                        Self::dump_result(args, control, &state.code[0], idx, &plan)?;
                    }
                    return Ok(JobCB::Null);
                }

                if !matches!(&compile_res.1, FResult::InternalCompileError(..))
                    && !matches!(&compile_res.1, FResult::HangOnCompile)
                {
                    return Ok(JobCB::Null);
                }

                if matches!(&compile_res.1, FResult::HangOnCompile) {
                    if args.skip_hang {
                        info!("\t\tHang: Skip hang on compile");
                        state.dropped = true;
                        return Ok(JobCB::Null);
                    }
                    if !Self::should_keep(args, keep_override, ResultKind::Hang) {
                        state.dropped = true;
                        return Ok(JobCB::Null);
                    }
                    let plan = DumpPlan {
                        kind: ResultKind::Hang,
                        prefix: "hang".to_string(),
                        compile_args: compile_args_for_dump,
                        extra_comment: None,
                        can_dump: true,
                    };
                    if has_dump_job {
                        state.dump_plan = Some(plan);
                    } else {
                        Self::dump_result(args, control, &state.code[0], idx, &plan)?;
                    }
                    return Ok(JobCB::InformICE(false));
                }

                let mut filter_pass = 0;
                for filter in filters {
                    let mut lock = filter.lock().map_err(|s| s.to_string())?;
                    if !lock.filter(&compile_res.1) {
                        filter_pass += 1;
                        let _ = lock.add(&compile_res.1);
                    }
                }
                if filter_pass == 0 {
                    info!("\t\tICE already exists, filtering out");
                    state.dropped = true;
                    let plan = DumpPlan {
                        kind: ResultKind::Ice,
                        prefix: "ice".to_string(),
                        compile_args: compile_args_for_dump,
                        extra_comment: Some("Duplicated ICE (filtered out)".to_string()),
                        can_dump: false,
                    };
                    if has_dump_job {
                        state.dump_plan = Some(plan);
                    }
                    return Ok(JobCB::InformICE(true));
                }
                info!("\t\tICE: New ICE found");

                if !Self::should_keep(args, keep_override, ResultKind::Ice) {
                    state.dropped = true;
                    return Ok(JobCB::InformICE(false));
                }

                let mut plan = DumpPlan {
                    kind: ResultKind::Ice,
                    prefix: "ice".to_string(),
                    compile_args: compile_args_for_dump.clone(),
                    extra_comment: None,
                    can_dump: true,
                };

                if args.use_unstable {
                    let flags = FEATURES.iter().map(|x| x.to_string()).collect::<Vec<_>>();
                    let extra_ref: Vec<&str> = extra_owned.iter().map(String::as_str).collect();
                    let minimized_flag = if self.rustc_toolchain.as_deref().is_some() {
                        filter_flags_with_toolchain::<RustcFuzzer>(
                            self.rustc_toolchain.as_deref(),
                            flags,
                            &state.code[0],
                            &self.tmp_source,
                            &self.tmp_bin,
                            &extra_ref,
                        )?
                    } else {
                        filter_flags::<RustcFuzzer>(
                            flags,
                            &state.code[0],
                            &self.tmp_source,
                            &self.tmp_bin,
                            &extra_ref,
                        )?
                    };
                    plan.extra_comment = Some(format!("Compile Flags: {}", minimized_flag.join(" ")));
                }
                if has_dump_job {
                    state.dump_plan = Some(plan);
                } else {
                    Self::dump_result(args, control, &state.code[0], idx, &plan)?;
                }
                return Ok(JobCB::InformICE(false));
            }
            JobType::Dump => {
                if let Some(plan) = state.dump_plan.as_ref() {
                    if plan.can_dump
                        && Self::should_keep(args, keep_override, plan.kind)
                        && !state.code.is_empty()
                    {
                        let mut plan = plan.clone();
                        if state.dump_plan_written > 0 {
                            let mode = match self.dump_mode {
                                DumpMode::Raw => "raw",
                                DumpMode::Pretty => "pretty",
                            };
                            plan.prefix = format!("{}_{}", plan.prefix, mode);
                        }
                        let code = Self::maybe_pretty_code(&state.code[0], self.dump_mode);
                        Self::dump_result(args, control, &code, idx, &plan)?;
                        state.dump_plan_written += 1;
                    }
                } else {
                    let p = args.output.join(format!("gen_{step_idx}_{idx}.rs"));
                    info!(
                        "\t\tDump to: {}",
                        p.as_os_str().to_str().unwrap_or_default()
                    );
                    let mut f = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(&p)?;
                    let mut flag = false;
                    for i in &state.code {
                        let out = Self::maybe_pretty_code(i, self.dump_mode);
                        if flag {
                            f.write_all(b"<|[Break Here]|>")?;
                        }
                        f.write_all(&out)?;
                        flag = true;
                    }
                }
            }
        }
        Ok(JobCB::Null)
    }
}

#[derive(Clone)]
struct JobHolder {
    jobs: Vec<Job>,
    filters: Vec<(FilterJob, SharedFilter)>,
    has_dump_job: bool,
    keep_override: Option<Vec<ResultKind>>,
    filter_override: Option<Vec<FilterJob>>,
}
impl JobHolder {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            filters: Vec::new(),
            has_dump_job: false,
            keep_override: None,
            filter_override: None,
        }
    }
    pub fn deep_clone(&self) -> Self {
        let mut jobs = Vec::new();
        for job in &self.jobs {
            jobs.push(job.deep_clone());
        }

        let filters = self.filters.clone();
        Self {
            jobs,
            filters,
            has_dump_job: self.has_dump_job,
            keep_override: self.keep_override.clone(),
            filter_override: self.filter_override.clone(),
        }
    }

    fn effective_filters(&self) -> Vec<SharedFilter> {
        let Some(selected) = &self.filter_override else {
            return self.filters.iter().map(|(_, f)| f.clone()).collect();
        };
        self.filters
            .iter()
            .filter(|(k, _)| selected.contains(k))
            .map(|(_, f)| f.clone())
            .collect()
    }

    pub fn add_job(&mut self, args: &Args, job: FuzzerJob) -> Result<(), Box<dyn Error>> {
        match job.stage {
            StageKind::Builtin(BuiltinStage::Filter) => {
                let expr = job
                    .task_arg
                    .as_deref()
                    .ok_or("filter task requires args, e.g. gate:filter:query-stack+panic-func")?;
                let (filters, keeps) = parse_filter_expr(expr).map_err(|e| e.to_string())?;
                if let Some(v) = filters {
                    self.filter_override = Some(v);
                }
                if let Some(v) = keeps {
                    self.keep_override = Some(v);
                }
                Ok(())
            }
            StageKind::Builtin(BuiltinStage::Dump) => {
                let fuzzer = Box::new(NoopFuzzer) as Box<dyn Fuzzer>;
                let dump_mode_name = job
                    .task_arg
                    .as_deref()
                    .or(Some(job.task_name.as_str()));
                let dump_mode = match dump_mode_name {
                    Some(s) if s.eq_ignore_ascii_case("pretty") => DumpMode::Pretty,
                    _ => DumpMode::Raw,
                };
                let job = Job::new(fuzzer, JobType::Dump, dump_mode, None);
                self.has_dump_job = true;
                self.jobs.push(job);
                Ok(())
            }
            StageKind::Job(job_type) => {
                let fuzzer_ty = job.fuzzer.ok_or("Missing tasker/fuzzer for this task")?;
                let fuzzer = fuzzer_ty.new_fuzzer(args)?;

                let rustc_toolchain = if job.tasker.eq_ignore_ascii_case("rustc")
                    && matches!(job_type, JobType::Fuzz)
                {
                    job.task_arg.clone()
                } else {
                    None
                };

                let job = Job::new(fuzzer, job_type, DumpMode::Raw, rustc_toolchain);
                self.jobs.push(job);
                Ok(())
            }
        }
    }

    pub fn add_filter(&mut self, kind: FilterJob, filter: Box<dyn ICEFilter>) -> Result<(), Box<dyn Error>> {
        let filter = Arc::new(Mutex::new(filter));
        self.filters.push((kind, filter));
        Ok(())
    }

    pub fn do_once(&mut self, args: &Args, control: &RunControl, idx: usize) -> Result<(), Box<dyn Error>> {
        let mut state = PipelineState::new();
        let mut filters = self.effective_filters();
        for (sidx, job) in self.jobs.iter().enumerate() {
            if state.dropped {
                break;
            }
            let cb = job.do_once(
                args,
                control,
                self.has_dump_job,
                self.keep_override.as_deref(),
                idx,
                sidx,
                &mut state,
                &mut filters,
            )?;
            match cb {
                JobCB::Null => {}
                JobCB::InformICE(dup) => {
                    for i in 0..sidx {
                        self.jobs[i]
                            .fuzzer
                            .lock()
                            .unwrap()
                            .inform_ice(&state.code[0], dup)?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn run(
    args: &Args,
    idx: Arc<AtomicUsize>,
    control: Arc<RunControl>,
    mut holder: JobHolder,
) -> Result<(), Box<dyn Error>> {
    loop {
        if control.should_stop(args) {
            STOP.store(true, Ordering::Relaxed);
            break;
        }
        control.inc_iter();
        let cidx = idx.fetch_add(1, Ordering::Relaxed);
        if let Some(max_iter) = args.max_iter
            && cidx >= max_iter
        {
            break;
        }
        if control.output_target_reached(args) {
            break;
        }
        if STOP.load(Ordering::Relaxed) {
            break;
        }

        info!("Current index: {cidx}");

        holder.do_once(args, &control, cidx)?;

        if control.should_stop(args) {
            STOP.store(true, Ordering::Relaxed);
            break;
        }
    }
    info!("Work thread exited!");
    Ok(())
}

fn check_stop(mut signals: Signals) {
    if let Some(sig) = signals.forever().next() {
        match sig {
            SIGINT | SIGTERM => {
                info!("Received signal: {sig}");
                STOP.store(true, Ordering::Relaxed);
            }
            _ => unreachable!(),
        }
    }
    info!("Signal thread exited!");
}

pub fn execute(args: &'static Args) -> Result<(), Box<dyn Error>> {
    STOP.store(false, Ordering::Relaxed);

    let mut worker = JobHolder::new();
    for f in &DEFAULT_FILTERS {
        info!("Creating filter: {:?}", f);
        let mut filter = f.new_filter(args)?;
        filter.import(args)?;
        info!(
            "Filter imported {}",
            std::any::type_name_of_val(filter.as_ref())
        );
        worker.add_filter(f.clone(), filter)?;
    }

    for job in &args.jobs {
        worker.add_job(args, job.clone())?;
        info!("Job added {:#?}", job);
    }

    create_dir_all(&args.output)?;
    info!("Begin generating...");

    let idx: Arc<AtomicUsize> = Arc::new(0.into());
    let control = Arc::new(RunControl::new());

    if args.nthread <= 1 {
        let idx = idx.clone();
        let control = control.clone();
        let worker = worker.clone();
        let handle = thread::Builder::new()
            .name("Worker-0".to_string())
            .stack_size(STACK_SIZE)
            .spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                let begin_time = Instant::now();
                let res = run(args, idx, control, worker).map_err(|e| e.to_string().into());
                let duration = begin_time.elapsed();
                warn!("Total Execution Time: {:?}", duration);
                res
            })?;
        let sig = Signals::new([SIGINT, SIGTERM])?;
        let _ = thread::spawn(move || {
            check_stop(sig);
        });
        thread::sleep(std::time::Duration::from_secs(1));
        let re = handle.join();
        let re = match re {
            Ok(re) => re,
            Err(e) => {
                error!("Thread atexit error: {e:?}");
                Ok(())
            }
        };
        if let Err(e) = re {
            error!("Thread error: {e}");
        }
    } else {
        let mut handles = Vec::default();
        let barrier = Arc::new(Barrier::new(args.nthread));
        let mut i = 0;

        info!(
            "Due to multi-thread needs to copy some data, the spawn procedure could be as slow as the init process :("
        );
        info!("Please wait patiently~ :P");

        while i < args.nthread {
            let worker_in_set = worker.deep_clone();
            for _ in 0..THREAD_SET_CNT {
                if i >= args.nthread {
                    break;
                }
                let worker_scope = worker_in_set.clone();
                let idx = idx.clone();
                let control = control.clone();
                let barrier = barrier.clone();
                let handle = thread::Builder::new()
                    .name(format!("Worker-{i}"))
                    .stack_size(STACK_SIZE)
                    .spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                        barrier.wait();
                        run(args, idx, control, worker_scope).map_err(|e| e.to_string().into())
                    })?;
                handles.push(handle);
                info!("Thread {i} spawned~");
                i += 1;
            }
        }
        info!("Begin to execute w~");
        info!("Press Ctrl+C to stop");
        let sig = Signals::new([SIGINT, SIGTERM])?;
        let _ = thread::spawn(move || {
            check_stop(sig);
        });
        thread::sleep(std::time::Duration::from_secs(1));

        while let Some(handle) = handles.pop() {
            let res = handle.join();
            let res = match res {
                Ok(res) => res,
                Err(e) => {
                    error!("Thread atexit error: {e:?}");
                    Ok(())
                }
            };
            if let Err(e) = res {
                error!("Thread error: {e}");
            }
        }
        info!("All threads exited!");
    }

    let (ice_cnt, success_cnt, compile_error_cnt, dumped_cnt, iter_cnt) = control.snapshot();
    info!(
        "Result summary: ice={ice_cnt}, success={success_cnt}, compile-error={compile_error_cnt}, dumped={dumped_cnt}, iter={iter_cnt}"
    );
    if control.timeout_reached(args) {
        warn!("Stopped due to timeout_sec={}", args.timeout_sec.unwrap_or_default());
    } else if control.output_target_reached(args) {
        info!("Stopped because dumped outputs reached target: loopcnt={}", args._loopcnt);
    } else if control.iter_limit_reached(args) {
        info!("Stopped because max_iter reached: {:?}", args.max_iter);
    }

    for (_, filter) in &worker.filters {
        let lock = filter.lock().map_err(|s| s.to_string())?;
        lock.export(args)?;
    }
    info!("Filter exported");

    Ok(())
}
