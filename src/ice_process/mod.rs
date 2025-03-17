use dyn_clone::DynClone;

use crate::fuzz::fuzzbase::FResult;

pub mod querystack;
pub mod panicfunc;

#[allow(unused)]
pub trait ICEFilter: Send + Sync + DynClone {
    fn filter(&self, info: &FResult) -> bool;
    fn add(&mut self, info: &FResult) -> bool;
    fn reset(&mut self);
}

#[derive(Default, Clone)]
pub struct DummyFilter {}
impl DummyFilter {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn ICEFilter> {
        Box::new(Self {})
    }
}
impl ICEFilter for DummyFilter {
    fn filter(&self, _: &FResult) -> bool {
        true
    }
    fn add(&mut self, _: &FResult) -> bool {
        true
    }
    fn reset(&mut self) {}
}
