use std::error::Error;

use syn::{visit_mut::VisitMut, Expr};

use crate::{debug, fuzz::strategy::FuzzerStrategyImpl, register_strategy, use_nodetype};

use_nodetype!(synfuzz, Expr);
use_nodetype!(synfuzz, Ident);

pub struct ExprBinaryStrategy;

impl FuzzerStrategyImpl<Expr> for ExprBinaryStrategy {
    fn do_fuzz(&self, v: &mut dyn VisitMut, _node: &mut Expr) -> Result<(), Box<dyn Error>> {
        debug!("Fuzzing binary expression");
        let node = if let Expr::Binary(node) = _node {
            node
        } else {
            return Ok(());
        };
        if let syn::BinOp::Add(_) = node.op {
            node.op = syn::BinOp::Mul(Default::default());
        }
        v.visit_attributes_mut(&mut node.attrs);
        v.visit_expr_mut(&mut *node.left);
        v.visit_bin_op_mut(&mut node.op);
        v.visit_expr_mut(&mut *node.right);
        Ok(())
    }
}

register_strategy!(ExprBinaryStrategy, Expr, Expr::Binary(_));
