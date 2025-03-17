use std::{env, error::Error, path::PathBuf};

use async_openai::{config::OpenAIConfig, types::{CreateChatCompletionRequest, CreateCompletionRequest}, Client};

use crate::{conf::Args, fuzz::fuzzbase::Fuzzer};

#[derive(Clone)]
pub struct ModelFuzzer {
}
impl ModelFuzzer {
    #[allow(clippy::new_ret_no_self)]
    #[allow(unused)]
    pub fn new(_: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        unimplemented!()
    }
}
impl Fuzzer for ModelFuzzer {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {

        unimplemented!()
    }
}
