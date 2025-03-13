use core::str;
use ouroboros::self_referencing;
use std::{collections::HashMap, error::Error, ffi::OsStr, fs::read, path::PathBuf, sync::LazyLock};
use tree_sitter::{Language, Tree};
use tree_splicer::{
    node_types::NodeTypes,
    splice::{Config, Splicer},
};
use walkdir::WalkDir;

use crate::{info, util::glob_next};

use super::fuzzbase::Fuzzer;

static NODE_TYPES: LazyLock<NodeTypes> =
    LazyLock::new(|| NodeTypes::new(tree_sitter_rust::NODE_TYPES).unwrap());
static LANGUAGE: LazyLock<Language> = LazyLock::new(tree_sitter_rust::language);

static EXCEPTIONS: &[&str] = &[
    // runtime
    "tests/ui/closures/issue-72408-nested-closures-exponential.rs",
    "tests/ui/issues/issue-74564-if-expr-stack-overflow.rs",
    "library/stdarch/crates/core_arch/src/mod.rs", //10+ mins
    // memory
    "tests/ui/issues/issue-50811.rs",
    "tests/ui/issues/issue-29466.rs",
    "src/tools/miri/tests/run-pass/float.rs",
    "tests/ui/numbers-arithmetic/saturating-float-casts-wasm.rs",
    "tests/ui/numbers-arithmetic/saturating-float-casts-impl.rs",
    "tests/ui/numbers-arithmetic/saturating-float-casts.rs",
    "tests/ui/wrapping-int-combinations.rs",
    // glacier/memory/time:
    "fixed/23600.rs",
    "23600.rs",
    "fixed/71699.rs",
    "71699.rs",
    // runtime
    "library/stdarch/crates/core_arch/src/x86/avx512bw.rs",
    "library/stdarch/crates/core_arch/src/x86/mod.rs",
    // 3.5 hours when reporting errors :(
    "library/stdarch/crates/core_arch/src/lib.rs",
    // memory 2.0
    "tests/run-make-fulldeps/issue-47551/eh_frame-terminator.rs",
    // infinite recursion in rustdoc, can take tens of minutes in ci
    "tests/ui/recursion/issue-38591-non-regular-dropck-recursion.rs",
    "tests/ui/dropck/dropck_no_diverge_on_nonregular_2.rs",
    "tests/ui/dropck/dropck_no_diverge_on_nonregular_1.rs",
    // 900 mb output, can take 5-10 minutes
    "tests/run-make-fulldeps/issue-47551/eh_frame-terminator.rs",
    // very slow
    "library/stdarch/crates/core_arch/src/x86/mod.rs",
    "library/core/src/lib.rs",
    "library/stdarch/crates/core_arch/src/mod.rs",
    "compiler/rustc_middle/src/lib.rs",
    "library/stdarch/crates/core_arch/src/x86/avx512f.rs",
    "tests/ui/structs-enums/struct-rec/issue-84611.rs",
    "tests/ui/structs-enums/struct-rec/issue-74224.rs",
    "tests/ui/dropck/dropck_no_diverge_on_nonregular_3.rs",
    "library/portable-simd/crates/core_simd/src/lib.rs", // 12+ minutes
    "tests/ui-fulldeps/myriad-closures.rs",
    "src/tools/miri/tests/pass/float.rs",
    "library/stdarch/crates/core_arch/src/arm_shared/neon/generated.rs",
    "library/stdarch/crates/core_arch/src/aarch64/mod.rs",
    "library/stdarch/crates/core_arch/src/aarch64/neon/generated.rs",
    "library/stdarch/crates/core_arch/src/aarch64/neon/mod.rs",
    "src/tools/cargo/tests/testsuite/main.rs",
    "src/tools/clippy/clippy_lints/src/lib.rs",
    "library/stdarch/crates/stdarch-gen/src/main.rs",
    "src/tools/rust-analyzer/crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/mod.rs",
    "src/tools/rust-analyzer/crates/proc-macro-srv/src/abis/abi_1_63/proc_macro/mod.rs",
    "tests/ui/issues/issue-22638.rs",
    "tests/ui/issues/issue-72933-match-stack-overflow.rs",
    "tests/ui/recursion/issue-86784.rs",
    "tests/ui/associated-types/issue-67684.rs",
];

fn parse(s: &str) -> Result<Tree, Box<dyn Error>> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(*LANGUAGE)?;
    let res = parser.parse(s, None).ok_or("tree-sitter parse failed!")?;
    Ok(res)
}

fn parse_dir(p: &Vec<PathBuf>) -> Result<HashMap<String, (Vec<u8>, Tree)>, Box<dyn Error>> {
    let mut res = HashMap::new();
    for d in p {
        info!("Loading dir {}", d.as_os_str().to_str().unwrap_or_default());
        for f in WalkDir::new(d) {
            let f = f?;
            if f.file_type().is_dir() {
                continue;
            }
            if f.path().extension() != Some(OsStr::new("rs")) {
                continue;
            }
            if EXCEPTIONS.iter().any(|s| f.path().ends_with(s)) {
                continue;
            }
            let path = f.path();
            let s = read(path)?;
            let tree = parse(str::from_utf8(&s)?)?;
            let path = path
                .as_os_str()
                .to_str()
                .ok_or("Path not stringable")?
                .to_string();
            res.insert(path, (s, tree));
        }
    }
    Ok(res)
}

fn do_splicer(
    trees: &HashMap<String, (Vec<u8>, Tree)>,
    seed: Option<u64>,
) -> Result<Splicer, Box<dyn Error>> {
    let seed = match seed {
        Some(s) => s,
        None => glob_next(),
    };
    let config = Config {
        chaos: 5,
        deletions: 5,
        language: *LANGUAGE,
        inter_splices: 16,
        max_size: 1024 * 1024, // 1M
        node_types: NODE_TYPES.clone(),
        reparse: 2,
        seed,
    };
    let splicer = Splicer::new(config, trees).ok_or("Init splicer failed, no files.")?;
    Ok(splicer)
}


#[self_referencing]
struct SplicerFuzzerHolder {
    trees: Box<HashMap<String, (Vec<u8>, Tree)>>,
    #[borrows(trees)]
    #[not_covariant]
    splicer: Splicer<'this>,
}

pub struct SplicerFuzzer {
    inner: SplicerFuzzerHolder,
}

impl SplicerFuzzer {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(dir: &Vec<PathBuf>) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let trees = parse_dir(dir)?;
        let trees = Box::new(trees);
        let res = SplicerFuzzerHolderBuilder {
            trees,
            splicer_builder: |t| do_splicer(t, Some(glob_next())).unwrap(),
        }
        .build();
        let res = Self { inner: res };
        Ok(Box::new(res))
    }
}

impl Fuzzer for SplicerFuzzer {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let code = self
            .inner
            .with_mut(|field| field.splicer.next())
            .ok_or("Ref err?")?;
        Ok(code)
    }
}
