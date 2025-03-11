use std::error::Error;
use syn::{Expr, ExprLoop};

use crate::{
    do_gen_name, fuzz::strategy::FuzzerStrategyImpl, gen_token, register_strategy, strategy::{info::env::ASTEnv, other::lifetime::gen_label}, use_nodetype
};

use_nodetype!(synfuzz, Block);
use_nodetype!(synfuzz, Expr);

#[derive(Default)]
pub struct ExprLoopStrategy;
impl FuzzerStrategyImpl<Expr> for ExprLoopStrategy {
    fn do_gen(&mut self) -> Result<Expr, Box<dyn Error>> {
        let e = ExprLoop {
            attrs: vec![],
            label: gen_label(),
            loop_token: gen_token!(loop),
            body: do_gen_name!(Block)?.do_gen()?,
        };
        Ok(Expr::Loop(e))
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |e| 1. / (50 * e.nested + 5 * e.expr_nest) as f64
    }
}

register_strategy!(ExprLoopStrategy, Expr, Expr::Loop(_));
