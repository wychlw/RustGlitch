use std::error::Error;
use syn::{Type, TypeNever};

use crate::{
    fuzz::strategy::FuzzerStrategyImpl, gen_token, register_strategy, strategy::info::env::ASTEnv, use_nodetype
};

use_nodetype!(synfuzz, Type);

pub struct TypeNeverStrategy;
impl Default for TypeNeverStrategy {
    fn default() -> Self {
        Self
    }
}
impl FuzzerStrategyImpl<Type> for TypeNeverStrategy {
    fn do_gen(&mut self) -> Result<Type, Box<dyn Error>> {
        let res = TypeNever {
            bang_token: gen_token!(!)
        };
        Ok(Type::Never(res))
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |_| 1.
    }
}

register_strategy!(TypeNeverStrategy, Type, Type::Array(..));
