use std::sync::{Arc, Mutex};
use std::{env, error::Error, path::PathBuf};

use crate::{conf::Args, fuzz::fuzzbase::Fuzzer};
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, Special};
use llama_cpp_2::model::{LlamaModel, params};
use llama_cpp_2::sampling::LlamaSampler;
use ouroboros::self_referencing;

struct ModelHolder {
    model: LlamaModel,
    backend: LlamaBackend,
    ctx_param: LlamaContextParams,
}
unsafe impl Send for ModelHolder {}
unsafe impl Sync for ModelHolder {}

#[derive(Clone)]
pub struct ModelFuzzer {
    inner: Arc<Mutex<ModelHolder>>,
}
impl ModelFuzzer {
    #[allow(clippy::new_ret_no_self)]
    #[allow(unused)]
    pub fn new(args: &Args) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let backend = LlamaBackend::init()?;
        let params = if args.gpu {
            LlamaModelParams::default().with_n_gpu_layers(1000)
        } else {
            LlamaModelParams::default()
        };
        let model = LlamaModel::load_from_file(&backend, &args.model, &params)?;
        let ctx_params = LlamaContextParams::default();

        let res = ModelHolder {
            model,
            backend,
            ctx_param: ctx_params,
        };
        let res = Self {
            inner: Arc::new(Mutex::new(res)),
        };
        Ok(Box::new(res))
    }

    fn _gen_code(&self, prefix: &str, suffix: &str) -> String {
        const PROMPT: &str = "You are a rust professor aimed in finding bugs in rust compilers. You need to give rust code which makes rust compiler throw Internal Compiler Error. You can use any nightly feature and items in std crate. Generate codes as strange as possible, and contains various structures and features.";

        let code = format!(
            "{}\n<|fim_prefix|>{}<|fim_suffix|>{}<|fim_middle|>",
            PROMPT, prefix, suffix
        );
        code
    }
}
impl Fuzzer for ModelFuzzer {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let inner = self.inner.lock().map_err(|e| e.to_string())?;
        let mut ctx = inner
            .model
            .new_context(&inner.backend, inner.ctx_param.clone())?;

        let code_prefix = "fn main() {\n    let mut x = 0;\n";
        let code_suffix = "\n    println!(\"{}\", Wsshc);\n}\n";
        let code = self._gen_code(code_prefix, code_suffix);

        let tokens = inner.model.str_to_token(&code, AddBos::Always)?;
        let mut batch = LlamaBatch::new(512, 1);

        let last_index = tokens.len() as i32 - 1;
        for (i, token) in (0i32..).zip(tokens.into_iter()) {
            let is_last = i == last_index;
            batch.add(token, i, &[0], is_last)?;
        }
        ctx.decode(&mut batch)?;

        let max_len = 4096;
        let mut cnt = batch.n_tokens();

        let mut decoder = encoding_rs::UTF_8.new_decoder();
        let mut sampler = LlamaSampler::greedy();

        while cnt <= max_len {
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            sampler.accept(token);
            if token == inner.model.token_eos() {
                break;
            }

        }

        unimplemented!()
    }
}
