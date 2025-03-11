use proc_macro2::Span;
use std::error::Error;
use syn::{Ident, Type, TypePath};

use crate::{
    fuzz::{rand_choose::WeightedRand, strategy::FuzzerStrategyImpl}, register_strategy,
    strategy::info::env::ASTEnv,
    use_nodetype,
};

use_nodetype!(synfuzz, Type);

macro_rules! dty {
    ($t: tt) => {
        (Ident::new($t, Span::call_site()), 1)
    };
}

// Type Path only for default defined obj
pub struct TypePathStrategy {
    rnd: WeightedRand<Ident, u32>,
}
impl Default for TypePathStrategy {
    fn default() -> Self {
        let all_items = vec![
            dty!("i8"),
            dty!("i16"),
            dty!("i32"),
            dty!("i64"),
            dty!("u8"),
            dty!("u16"),
            dty!("u32"),
            dty!("u64"),
            dty!("str"),
            dty!("String"),
            dty!("Option"),
            dty!("Result")
        ];
        Self {
            rnd: WeightedRand::new(Some(all_items), None),
        }
    }
}
unsafe impl Send for TypePathStrategy {}
unsafe impl Sync for TypePathStrategy {}
impl FuzzerStrategyImpl<Type> for TypePathStrategy {
    fn do_gen(&mut self) -> Result<Type, Box<dyn Error>> {
        let p = self.rnd.rand()?;
        let res = TypePath {
            qself: None,
            path: p.0.clone().into(),
        };
        Ok(Type::Path(res))
    }
    fn gen_weight(&self) -> fn(&ASTEnv) -> f64 {
        |_| 2.
    }
}

register_strategy!(TypePathStrategy, Type, Type::Array(..));
