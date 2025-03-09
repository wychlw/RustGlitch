use std::error::Error;

use syn::visit_mut::VisitMut;

pub trait FuzzerStrategyImpl<Node> {
    fn do_fuzz(&self, v: &mut dyn VisitMut, node: &mut Node) -> Result<(), Box<dyn Error>>;
}

#[doc(hidden)]
pub enum DoFuzzRes {
    Success,
    NoStreatgy,
}

#[doc(hidden)]
pub struct FuzzerStrategyHolder<Node> {
    pub inner: Box<DynFuzzerStrategyTyper<Node>>,
}
unsafe impl<Node> Send for FuzzerStrategyHolder<Node> {}
unsafe impl<Node> Sync for FuzzerStrategyHolder<Node> {}

#[doc(hidden)]
// should impl by macro
pub trait FuzzerStrategyTyper<Node>: FuzzerStrategyImpl<Node> {
    fn is_kind(&self, node: &Node) -> bool;
}

#[doc(hidden)]
pub type DynFuzzerStrategyTyper<Node> = dyn FuzzerStrategyTyper<Node> + Send + Sync + 'static;

#[macro_export]
macro_rules! register_nodetype {
    ($node_type: ident) => {
        #[allow(non_upper_case_globals)]
        pub static ${concat("STRATEGY_HOLDER_", $node_type)}: std::sync::LazyLock<std::sync::Mutex<Vec<$crate::fuzz::strategy::FuzzerStrategyHolder<$node_type>>>> =
            std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
        #[allow(non_snake_case)]
        pub fn ${concat("get_strategies_", $node_type)}() -> Result<std::sync::MutexGuard<'static, Vec<$crate::fuzz::strategy::FuzzerStrategyHolder<$node_type>>>, Box<dyn Error>> {
            if let Ok(guard) = ${concat("STRATEGY_HOLDER_", $node_type)}.lock() {
                Ok(guard)
            } else {
                return Err("Failed to lock strategy holder".into());
            }
        }
        #[allow(non_snake_case)]
        pub fn ${concat("do_fuzz_", $node_type)}(v: &mut dyn VisitMut, node: &mut $node_type) -> Result<$crate::fuzz::strategy::DoFuzzRes, Box<dyn Error>> {
            let strategies = ${concat("get_strategies_", $node_type)}()?;
            for strategy in strategies.iter() {
                let inner = &strategy.inner;
                if !inner.is_kind(node) {
                    continue;
                }
                inner.do_fuzz(v, node)?;
                return Ok($crate::fuzz::strategy::DoFuzzRes::Success);
            }
            Ok($crate::fuzz::strategy::DoFuzzRes::NoStreatgy)
        }
};
}

#[macro_export]
macro_rules! use_nodetype {
    ($file: ident, $node_type: ident) => {
        #[allow(unused_imports)]
        use $crate::fuzz::$file::${concat("STRATEGY_HOLDER_", $node_type)};
        #[allow(unused_imports)]
        use $crate::fuzz::$file::${concat("get_strategies_", $node_type)};
        #[allow(unused_imports)]
        use $crate::fuzz::$file::${concat("do_fuzz_", $node_type)};
    };
}

#[macro_export]
macro_rules! do_fuzz {
    ($node_type: ident, $self: ident, $node: ident) => {
        ${concat("do_fuzz_", $node_type)}($self, $node)
    };
}

#[macro_export]
macro_rules! get_strategies {
    ($node_type: ident) => {
        ${concat("get_strategies_", $node_type)}()
    };
}

#[macro_export]
macro_rules! register_strategy {
    ($name: ident, $node_type: ident, $node_kind: pat $(if $guard:expr)? $(,)?) => {
        impl $crate::fuzz::strategy::FuzzerStrategyTyper<$node_type> for $name {
            fn is_kind(&self, node: &$node_type) -> bool {
                matches!(node, $node_kind)
            }
        }

        #[allow(non_snake_case)]
        #[ctor::ctor]
        fn ${concat("__strategy_reg_ctor_", $name)}() {
            $crate::info!("Registered strategy: {}", stringify!($name));
            let strategy = $crate::fuzz::strategy::FuzzerStrategyHolder {inner: Box::new($name)};
            if let Ok(mut guard) = ${concat("STRATEGY_HOLDER_", $node_type)}.lock() {
                guard.push(strategy);
            }
        }
    };
}
