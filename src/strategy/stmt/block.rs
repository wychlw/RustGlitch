use std::error::Error;
use syn::{Block, Stmt, token::Brace};

use crate::{
    do_gen_name,
    fuzz::strategy::FuzzerStrategyImpl,
    register_strategy_no_kind,
    strategy::{
        consts::BLOCK_STMT_R,
        info::env::{ASTEnv, nest_env, unnest_env},
    },
    use_nodetype,
    util::glob_range,
};

use_nodetype!(synfuzz, Block);
use_nodetype!(synfuzz, Expr);
use_nodetype!(synfuzz, Stmt);

#[derive(Default)]
pub struct BlockStrategy {}
impl FuzzerStrategyImpl<Block> for BlockStrategy {
    fn do_gen(&mut self) -> Result<Block, Box<dyn Error>> {

        let stmt_cnt = glob_range(BLOCK_STMT_R);

        nest_env()?;

        let mut stmts: Vec<Stmt> = vec![];
        for _ in 0..stmt_cnt {
            stmts.push(do_gen_name!(Stmt)?.do_gen()?);
        }

        unnest_env()?;

        Ok(Block {
            brace_token: Brace::default(),
            stmts,
        })
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |_| 0.
    }
}

register_strategy_no_kind!(BlockStrategy, Block, 50);
