use std::error::Error;
use syn::{Local, LocalInit, Stmt};

use crate::{
    do_gen_name,
    fuzz::strategy::FuzzerStrategyImpl,
    gen_token, register_strategy,
    strategy::{consts::LET_ELSE_B, info::env::ASTEnv},
    use_nodetype,
    util::glob_range,
};

use_nodetype!(synfuzz, Stmt);
use_nodetype!(synfuzz, Expr);
use_nodetype!(synfuzz, Pat);

pub struct StmtLocalStrategy;
impl Default for StmtLocalStrategy {
    fn default() -> Self {
        Self
    }
}
impl FuzzerStrategyImpl<Stmt> for StmtLocalStrategy {
    fn do_gen(&mut self) -> Result<Stmt, Box<dyn Error>> {
        let diverge = if LET_ELSE_B > glob_range(0. ..1.) {
            Some((gen_token!(else), do_gen_name!(Expr)?.do_gen()?.into()))
        } else {
            None
        };
        let init = LocalInit {
            eq_token: gen_token!(=),
            expr: do_gen_name!(Expr)?.do_gen()?.into(),
            diverge,
        };
        let res = Local {
            attrs: vec![],
            let_token: gen_token!(let),
            pat: do_gen_name!(Pat)?.do_gen()?,
            init: Some(init),
            semi_token: gen_token!(;),
        };
        let res = Stmt::Local(res);
        Ok(res)
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |_| 1.
    }
}

register_strategy!(StmtLocalStrategy, Stmt, Stmt::Local(..));
