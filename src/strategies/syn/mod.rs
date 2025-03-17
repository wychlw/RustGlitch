use quote::ToTokens;
use strategy::item::itemfn::do_gen_main;
use std::error::Error;

mod strategy;

use syn::{
    Block, Expr, Item, Pat, Stmt, Type,
};
use syn::File;

use crate::conf::Args;
use crate::util::glob_range;
use crate::{
    do_gen_name, register_nodetype
};

use crate::fuzz::fuzzbase::Fuzzer;

#[derive(Clone)]
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
    #[allow(unused)]
    pub fn new(_: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let res = Self {};
        Ok(Box::new(res))
    }
}

impl Fuzzer for SynFuzzer {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
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

        let stream = f.to_token_stream();
        let code = stream.to_string();
        let code = code.into_bytes();
        Ok(code)
    }
}
