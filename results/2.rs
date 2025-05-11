#![feature(generic_const_exprs)]
trait T{}
trait V{}
impl<const N: ()> T for [(); N::<&mut V>] {} 

// https://github.com/rust-lang/rust/issues/140275
