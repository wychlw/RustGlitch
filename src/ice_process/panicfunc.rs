use std::{collections::HashSet, error::Error};

use crate::fuzz::fuzzbase::FResult;

use super::ICEFilter;

static QUERY_PANIC_BEGIN: &str = "thread 'rustc' panicked at ";

fn filter_panic_file(msg: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let msg_str = str::from_utf8(msg)?;
    let msg: Vec<_> = msg_str
        .lines()
        .filter_map(|s| -> Option<Vec<u8>> {
            let begin = s.find(QUERY_PANIC_BEGIN)? + QUERY_PANIC_BEGIN.len();
            let end = s.len() - 1;
            let b = &s.as_bytes()[begin..end];
            Some(b.to_vec())
        })
        .collect();
    Ok(msg.last().map(|v| v.to_owned()))
}

#[derive(Clone)]
pub struct PanicFuncFilter {
    existed: HashSet<Vec<u8>>,
}
impl PanicFuncFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn ICEFilter> {
        Box::new(Self {
            existed: HashSet::default(),
        })
    }
}
impl ICEFilter for PanicFuncFilter {
    fn filter(&self, info: &FResult) -> bool {
        let info = match info {
            FResult::InternalCompileError(x) => x,
            _ => return false,
        };
        let msg = filter_panic_file(&info.stderr);
        let msg = match msg {
            Ok(Some(x)) => x,
            _ => return false,
        };
        self.existed.contains(&msg)
    }
    fn add(&mut self, info: &FResult) -> bool {
        let info = match info {
            FResult::InternalCompileError(x) => x,
            _ => return false,
        };
        let msg = filter_panic_file(&info.stderr);
        let msg = match msg {
            Ok(Some(x)) => x,
            _ => return false,
        };
        self.existed.insert(msg)
    }
    fn reset(&mut self) {
        self.existed.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use crate::{fuzz::fuzzbase::{DummyFuzzer, Fuzzer}, util::gen_alnum};

    use super::*;

    #[test]
    fn test_filter_panic_func() {
        let code = b"fn main() {break rust}";
        let tmp_file = temp_dir().join(gen_alnum(4));
        let tmp_out = temp_dir().join(gen_alnum(4));
        let res = DummyFuzzer::compile(code, &tmp_file, &tmp_out, &[]).unwrap();
        let res = match res.1 {
            FResult::InternalCompileError(x) => x,
            _ => unreachable!()
        };
        let res = filter_panic_file(&res.stderr).unwrap();

        let expected = b"compiler/rustc_hir_typeck/src/lib.rs:528:10".to_vec();

        let res = res.unwrap();
        assert_eq!(res, expected);
    }
}
