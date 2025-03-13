use quote::ToTokens;
use std::{
    error::Error,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use syn::{
    Block, Expr, Item, Pat, Stmt, Type,
    fold::{Fold, fold_expr},
};
use syn::{File, parse_file};

use crate::{
    do_fuzz_name, do_gen_name, error, register_nodetype, strategy::item::itemfn::do_gen_main,
    util::glob_range,
};

use super::{fuzzbase::Fuzzer, strategy::DoFuzzRes};

pub struct SynFuzzer {
}

register_nodetype!(Expr);
register_nodetype!(Block);
register_nodetype!(Stmt);
register_nodetype!(Item);
register_nodetype!(Pat);
register_nodetype!(Type);

impl SynFuzzer {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let res = Self;
        Ok(Box::new(res))
    }
}

impl Fuzzer for SynFuzzer {
    fn generate(&mut self) -> Result<(), Box<dyn Error>> {
        let item_cnt = glob_range(1..10);
        let mut item_v = Vec::new();
        for _ in 0..item_cnt {
            item_v.push(do_gen_name!(Item)?.do_gen()?);
        }
        item_v.push(do_gen_main()?);
        let f = File {
            shebang: None,
            attrs: vec![],
            items: item_v,
        };
        self.ast = Some(f);
        Ok(())
    }
    fn dump(&mut self, output: &Path) -> Result<(), Box<dyn Error>> {
        let stream = self.ast.to_token_stream();
        let code = stream.to_string();
        write(output, &code)?;
        // Command::new("rustfmt").arg(output).status()?;
        Ok(())
    }
}
