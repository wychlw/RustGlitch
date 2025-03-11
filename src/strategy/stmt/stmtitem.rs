use std::error::Error;
use syn::Stmt;

use crate::{
    do_gen_name, fuzz::strategy::FuzzerStrategyImpl, register_strategy, strategy::info::env::ASTEnv, use_nodetype,
};

use_nodetype!(synfuzz, Stmt);
use_nodetype!(synfuzz, Item);

pub struct StmtItemStrategy;
impl Default for StmtItemStrategy {
    fn default() -> Self {
        Self
    }
}
impl FuzzerStrategyImpl<Stmt> for StmtItemStrategy {
    fn do_gen(&mut self) -> Result<Stmt, Box<dyn Error>> {
        let stra = do_gen_name!(Item)?;
        let res = Stmt::Item(stra.do_gen()?);
        Ok(res)
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |_| 0. // todo: fix cause stack_overflow
    }
}

register_strategy!(StmtItemStrategy, Stmt, Stmt::Item(..));
