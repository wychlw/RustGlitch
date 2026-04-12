use std::{
    collections::HashSet,
    error::Error,
    fs::{self, read_to_string},
};

use crate::{conf::Args, debug, fuzz::fuzzbase::FResult};

use serde_json;

use super::ICEFilter;

static QUERY_PANIC_BEGIN: &str = "thread 'rustc' panicked at ";

fn filter_panic_file(msg: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let msg_str = str::from_utf8(msg)?;
    let msg: Vec<_> = msg_str
        .lines()
        .filter_map(|s| -> Option<_> {
            let begin = s.find(QUERY_PANIC_BEGIN)? + QUERY_PANIC_BEGIN.len();
            let end = s.len() - 1;
            let b = &s[begin..end];
            Some(b.as_bytes().to_vec())
        })
        .collect();
    Ok(msg.last().map(|v| v.to_owned()))
}

type FilterData = HashSet<Vec<u8>>;
#[derive(Clone, Debug)]
pub struct PanicFuncFilter {
    existed: FilterData,
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
        debug!("PanicFuncFilter filtering: {:#?}", msg);
        let msg = match msg {
            Ok(Some(x)) => x,
            _ => return false,
        };
        debug!("The panic function is: \n\t{}", str::from_utf8(&msg).unwrap_or_default());
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
    fn import(&mut self, args: &Args) -> Result<(), Box<dyn Error>> {
        let p = args.datas.join("func_filters.json");
        if !p.exists() {
            return Ok(());
        }
        let f = read_to_string(p)?;
        let datas: FilterData = serde_json::from_str(&f)?;
        debug!("PanicFunc importing panic filters: {}", datas.len());
        self.existed.extend(datas);
        Ok(())
    }
    fn export(&self, args: &Args) -> Result<(), Box<dyn Error>> {
        let datas = serde_json::to_string(&self.existed)?;
        let p = args.datas.join("func_filters.json");
        fs::write(p, datas)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_panic_func() {
        // This should not depend on the local rustc version producing an ICE.
        // We only test the stderr parsing logic here.
        let stderr = b"some prelude\nthread 'rustc' panicked at compiler/foo.rs:1:2:\nmore\nthread 'rustc' panicked at compiler/rustc_hir_typeck/src/lib.rs:528:10:\n";
        let res = filter_panic_file(stderr).unwrap().unwrap();
        assert_eq!(res, b"compiler/rustc_hir_typeck/src/lib.rs:528:10".to_vec());
    }
}
