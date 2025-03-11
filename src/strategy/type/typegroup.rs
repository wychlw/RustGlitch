use std::error::Error;
use syn::{token::Group, Type, TypeGroup};

use crate::{
    do_gen_name, fuzz::strategy::FuzzerStrategyImpl, register_strategy, strategy::info::env::ASTEnv, use_nodetype
};

use_nodetype!(synfuzz, Type);

pub struct TypeGroupStrategy;
impl Default for TypeGroupStrategy {
    fn default() -> Self {
        Self
    }
}
impl FuzzerStrategyImpl<Type> for TypeGroupStrategy {
    fn do_gen(&mut self) -> Result<Type, Box<dyn Error>> {
        let res = TypeGroup {
            group_token: Group::default(),
            elem: do_gen_name!(Type)?.do_gen()?.into(),
        };
        Ok(Type::Group(res))
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |e| 1. / (5 * e.type_nest) as f64
    }
}

register_strategy!(TypeGroupStrategy, Type, Type::Array(..));
