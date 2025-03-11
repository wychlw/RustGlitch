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
    ast: Option<File>,
}

register_nodetype!(Expr);
register_nodetype!(Block);
register_nodetype!(Stmt);
register_nodetype!(Item);
register_nodetype!(Pat);
register_nodetype!(Type);

impl Fold for SynFuzzer {
    fn fold_expr(&mut self, node: Expr) -> Expr {
        match do_fuzz_name!(Expr, self, ExprStrategy, node) {
            Ok(DoFuzzRes::Success(nd)) => nd,
            Ok(DoFuzzRes::NoStreatgy(nd)) => fold_expr(self, nd),
            Err(e) => {
                error!("Error: {:?}", e);
                panic!();
            }
        }
    }
}

impl SynFuzzer {
    #[allow(unused)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(file: &PathBuf, extra_args: &[String]) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let code = read_to_string(file)?;
        let ast = parse_file(&code)?;
        let res = Self { ast: Some(ast) };
        Ok(Box::new(res))
    }
}

impl Fuzzer for SynFuzzer {
    fn replace(&mut self) -> Result<(), Box<dyn Error>> {
        let ast = self.ast.take().ok_or("No AST")?;
        self.ast = Some(self.fold_file(ast));
        Ok(())
    }
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
