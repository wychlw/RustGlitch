use std::error::Error;
use syn::{
    fold::{fold_expr_binary, Fold}, BinOp, Expr, ExprBinary
};

use crate::{do_gen_name, fuzz::strategy::FuzzerStrategyImpl, register_strategy, strategy::info::env::ASTEnv, use_nodetype};

use_nodetype!(synfuzz, Expr);

#[derive(Default)]
pub struct ExprBinaryStrategy;
impl FuzzerStrategyImpl<Expr> for ExprBinaryStrategy {
    fn do_fuzz(&self, v: &mut dyn Fold, _node: Expr) -> Result<Expr, Box<dyn Error>> {
        let mut node = if let Expr::Binary(node) = _node {
            node
        } else {
            return Err("Not a binary expression".into());
        };
        if let BinOp::Add(_) = node.op {
            node.op = BinOp::Mul(Default::default());
        }
        let res = fold_expr_binary(v, node);
        Ok(Expr::Binary(res))
    }
    fn do_gen(&mut self) -> Result<Expr, Box<dyn Error>> {
        let le = do_gen_name!(Expr)?.do_gen()?;
        let re = do_gen_name!(Expr)?.do_gen()?;
        let new_bin = ExprBinary {
            attrs: Vec::new(),
            left: le.into(),
            op: BinOp::Add(Default::default()),
            right: re.into()
        };
        Ok(Expr::Binary(new_bin))
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |e| 1. / (5 * e.expr_nest) as f64
    }
}

register_strategy!(ExprBinaryStrategy, Expr, Expr::Binary(_));
