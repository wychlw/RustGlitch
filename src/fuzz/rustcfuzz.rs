use std::thread;
// use std::default;
use std::error::Error;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_fluent_macro;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;

use rustc_ast::{
    BinOpKind, Crate, Expr, ExprKind,
    mut_visit::{MutVisitor, walk_expr},
    ptr::P,
};
use rustc_ast_pretty::pprust;
use rustc_codegen_ssa::traits::CodegenBackend;
use rustc_data_structures::{jobserver, sync};
use rustc_driver::{
    Callbacks, Compilation, DEFAULT_LOCALE_RESOURCES, USING_INTERNAL_FEATURES, args,
    diagnostics_registry, handle_options, run_compiler,
};
use rustc_interface::{interface, interface::Compiler, passes};
use rustc_middle::{ty::CurrentGcx, util::Providers};
use rustc_session::{
    CompilerIO, EarlyDiagCtxt, Session, config,
    config::{ErrorOutputType, Input, OutFileName, build_session_options},
    filesearch,
    filesearch::sysroot_candidates,
};
use rustc_span::edition::Edition;
use rustc_span::source_map::{RealFileLoader, SourceMapInputs};

use crate::util::ForceSend;

use super::fuzz::Fuzzer;
pub struct RustCFuzzer {
    file: PathBuf,
    ast: Arc<Mutex<Option<Crate>>>,
}

pub struct SelfCompiler {
    pub sess: Session,
    pub codegen_backend: Box<dyn CodegenBackend>,
    pub override_queries: Option<fn(&Session, &mut Providers)>,
    pub current_gcx: CurrentGcx,
}

struct RustCFuzzerVisitor;
impl MutVisitor for RustCFuzzerVisitor {
    fn visit_expr(&mut self, ex: &mut P<Expr>) {
        if let ExprKind::Binary(op, lhs, rhs) = &mut ex.kind {
            if op.node == BinOpKind::Add {
                op.node = BinOpKind::Mul;
                // op.span = Span::default();
                self.visit_expr(lhs);
                self.visit_expr(rhs);
            }
        }
        walk_expr(self, ex);
    }
}

fn gen_compiler_config(args: &[String]) -> Result<interface::Config, Box<dyn Error>> {
    let mut default_early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    let at_args = args.get(1..).unwrap_or_default();
    let args = args::arg_expand_all(&default_early_dcx, at_args);
    let Some(matches) = handle_options(&default_early_dcx, &args) else {
        return Err("Failed to handle options".into());
    };
    let sopts = build_session_options(&mut default_early_dcx, &matches);
    let ice_file = Some(PathBuf::from("/home/lw/Work/rust/nfuzz/ice.txt"));
    let odir = matches.opt_str("out-dir").map(|o| PathBuf::from(&o));
    let ofile = matches.opt_str("o").map(|o| match o.as_str() {
        "-" => OutFileName::Stdout,
        path => OutFileName::Real(PathBuf::from(path)),
    });
    let config = interface::Config {
        opts: sopts,
        crate_cfg: matches.opt_strs("cfg"),
        crate_check_cfg: matches.opt_strs("check-cfg"),
        input: Input::File(PathBuf::new()),
        output_file: ofile,
        output_dir: odir,
        ice_file: ice_file,
        file_loader: None,
        locale_resources: DEFAULT_LOCALE_RESOURCES.to_vec(),
        lint_caps: Default::default(),
        psess_created: None,
        hash_untracked_state: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: diagnostics_registry(),
        using_internal_features: &USING_INTERNAL_FEATURES,
        expanded_args: args,
    };
    Ok(config)
}

pub const DEFAULT_STACK_SIZE: usize = 8 * 1024 * 1024;

fn run_in_thread_pool_with_globals<F: FnOnce(CurrentGcx) -> R + Send, R: Send>(
    edition: Edition,
    threads: usize,
    sm_inputs: SourceMapInputs,
    f: F,
) -> R {
    let thread_stack_size = DEFAULT_STACK_SIZE;

    let registry = sync::Registry::new(std::num::NonZero::new(threads).unwrap());

    let builder = thread::Builder::new()
        .name("rustc".to_string())
        .stack_size(thread_stack_size);

    thread::scope(|s| {
        let r = builder
            .spawn_scoped(s, move || {
                rustc_span::create_session_globals_then(edition, Some(sm_inputs), || {
                    let current_gcx = CurrentGcx::new();
                    registry.register();
                    f(current_gcx)
                })
            })
            .unwrap()
            .join();
        match r {
            Ok(v) => v,
            Err(e) => std::panic::resume_unwind(e),
        }
    })
}

fn run_compiler_no_abort<R: Send>(
    config: interface::Config,
    f: impl FnOnce(&SelfCompiler) -> R + Send,
) -> R {
    rustc_data_structures::sync::set_dyn_thread_safe_mode(config.opts.unstable_opts.threads > 1);

    let early_dcx = EarlyDiagCtxt::new(config.opts.error_format);
    let early_dcx_ref = &early_dcx;
    jobserver::initialize_checked(|err| {
        early_dcx_ref
            .early_struct_warn(err)
            .with_note("the build environment is likely misconfigured")
            .emit()
    });

    // crate::callbacks::setup_callbacks();

    let sysroot = filesearch::materialize_sysroot(config.opts.maybe_sysroot.clone());
    let target = config::build_target_config(&early_dcx, &config.opts.target_triple, &sysroot);
    let file_loader = config
        .file_loader
        .unwrap_or_else(|| Box::new(RealFileLoader));
    let path_mapping = config.opts.file_path_mapping();
    let hash_kind = config.opts.unstable_opts.src_hash_algorithm(&target);
    let checksum_hash_kind = config.opts.unstable_opts.checksum_hash_algorithm();

    run_in_thread_pool_with_globals(
        config.opts.edition,
        config.opts.unstable_opts.threads,
        SourceMapInputs {
            file_loader,
            path_mapping,
            hash_kind,
            checksum_hash_kind,
        },
        |current_gcx| {
            // The previous `early_dcx` can't be reused here because it doesn't
            // impl `Send`. Creating a new one is fine.
            let early_dcx = EarlyDiagCtxt::new(config.opts.error_format);

            let codegen_backend = match config.make_codegen_backend {
                None => rustc_interface::util::get_codegen_backend(
                    &early_dcx,
                    &sysroot,
                    config.opts.unstable_opts.codegen_backend.as_deref(),
                    &target,
                ),
                Some(make_codegen_backend) => make_codegen_backend(&config.opts),
            };

            let temps_dir = config
                .opts
                .unstable_opts
                .temps_dir
                .as_deref()
                .map(PathBuf::from);

            let bundle = match rustc_errors::fluent_bundle(
                config.opts.maybe_sysroot.clone(),
                sysroot_candidates().to_vec(),
                config.opts.unstable_opts.translate_lang.clone(),
                config
                    .opts
                    .unstable_opts
                    .translate_additional_ftl
                    .as_deref(),
                config.opts.unstable_opts.translate_directionality_markers,
            ) {
                Ok(bundle) => bundle,
                Err(e) => early_dcx.early_fatal(format!("failed to load fluent bundle: {e}")),
            };

            let mut locale_resources = config.locale_resources;
            locale_resources.push(codegen_backend.locale_resource());

            let mut sess = rustc_session::build_session(
                config.opts,
                CompilerIO {
                    input: config.input,
                    output_dir: config.output_dir,
                    output_file: config.output_file,
                    temps_dir,
                },
                bundle,
                config.registry,
                locale_resources,
                config.lint_caps,
                target,
                sysroot,
                rustc_interface::util::rustc_version_str().unwrap_or("unknown"),
                config.ice_file,
                config.using_internal_features,
                config.expanded_args,
            );

            codegen_backend.init(&sess);

            // let cfg = parse_cfg(sess.dcx(), config.crate_cfg);
            // let mut cfg = config::build_configuration(&sess, cfg);
            // util::add_configuration(&mut cfg, &mut sess, &*codegen_backend);
            // sess.psess.config = cfg;

            // let mut check_cfg = parse_check_cfg(sess.dcx(), config.crate_check_cfg);
            // check_cfg.fill_well_known(&sess.target);
            // sess.psess.check_config = check_cfg;

            if let Some(psess_created) = config.psess_created {
                psess_created(&mut sess.psess);
            }

            // if let Some(hash_untracked_state) = config.hash_untracked_state {
            //     let mut hasher = StableHasher::new();
            //     hash_untracked_state(&sess, &mut hasher);
            //     sess.opts.untracked_state_hash = hasher.finish()
            // }

            // // Even though the session holds the lint store, we can't build the
            // // lint store until after the session exists. And we wait until now
            // // so that `register_lints` sees the fully initialized session.
            // let mut lint_store = rustc_lint::new_lint_store(sess.enable_internal_lints());
            // if let Some(register_lints) = config.register_lints.as_deref() {
            //     register_lints(&sess, &mut lint_store);
            // }
            // sess.lint_store = Some(Arc::new(lint_store));

            // util::check_abi_required_features(&sess);

            let compiler = SelfCompiler {
                sess,
                codegen_backend,
                override_queries: config.override_queries,
                current_gcx,
            };

            let res = f(&compiler);

            // There are two paths out of `f`.
            // - Normal exit.
            // - Panic, e.g. triggered by `abort_if_errors` or a fatal error.
            //
            // We must run `finish_diagnostics` in both cases.
            // let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&compiler)));

            // compiler.sess.finish_diagnostics();

            // If error diagnostics have been emitted, we can't return an
            // error directly, because the return type of this function
            // is `R`, not `Result<R, E>`. But we need to communicate the
            // errors' existence to the caller, otherwise the caller might
            // mistakenly think that no errors occurred and return a zero
            // exit code. So we abort (panic) instead, similar to if `f`
            // had panicked.
            // if res.is_ok() {
            //     compiler.sess.dcx().abort_if_errors();
            // }

            // Also make sure to flush delayed bugs as if we panicked, the
            // bugs would be flushed by the Drop impl of DiagCtxt while
            // unwinding, which would result in an abort with
            // "panic in a destructor during cleanup".
            // compiler.sess.dcx().flush_delayed();

            // let res = match res {
            //     Ok(res) => res,
            //     // Resume unwinding if a panic happened.
            //     Err(err) => std::panic::resume_unwind(err),
            // };

            // let prof = compiler.sess.prof.clone();
            // prof.generic_activity("drop_compiler").run(move || drop(compiler));

            res
        },
    )
}

impl RustCFuzzer {
    #[allow(unused)]
    pub fn new(file: &PathBuf, extra_args: &[String]) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let res: Self = Self {
            file: file.clone(),
            ast: Arc::new(Mutex::new(None)),
        };

        let ast = res.ast.clone();
        let mut ast = ForceSend::new(ast);

        let args = vec!["rustc".to_string(), file.to_str().unwrap().to_string()];
        let args: Vec<String> = args
            .iter()
            .chain(extra_args.iter())
            .map(|s| s.clone())
            .collect();
        let mut config = gen_compiler_config(&args)?;
        config.input = Input::File(file.clone());

        run_compiler_no_abort(config, |compiler| {
            let sess = &compiler.sess;
            let krate = passes::parse(sess);
            let ast_inner = ast.deref_mut();
            ast_inner.lock().unwrap().replace(krate);
        });
        Ok(Box::new(res))
    }
}

impl Fuzzer for RustCFuzzer {
    fn replace(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ast_lock = self.ast.lock().unwrap();
        let mut ast = ast_lock.as_mut().unwrap();
        let mut visitor = RustCFuzzerVisitor;
        visitor.visit_crate(&mut ast);
        Ok(())
    }

    fn dump(&mut self, output: &PathBuf) -> Result<(), Box<dyn Error>> {
        let ast = self.ast.lock().unwrap().as_ref().unwrap().clone();
        struct RustCFuzzerDumpCB {
            ast: Crate,
            output: PathBuf,
        }
        unsafe impl Send for RustCFuzzerDumpCB {}
        impl Callbacks for RustCFuzzerDumpCB {
            fn after_crate_root_parsing(
                &mut self,
                _compiler: &Compiler,
                _krate: &mut Crate,
            ) -> Compilation {
                let code = pprust::crate_to_string_for_macros(&self.ast);
                std::fs::write(&self.output, code).unwrap();
                Compilation::Stop
            }
        }
        let mut cb = RustCFuzzerDumpCB {
            ast: ast,
            output: output.clone(),
        };
        let args = vec![
            "rustc".to_string(),
            self.file.to_str().unwrap().to_string(),
            "--edition".to_string(),
            "2024".to_string(),
        ];
        run_compiler(&args, &mut cb);
        Command::new("rustfmt").arg(output).status()?;
        Ok(())
    }
}
