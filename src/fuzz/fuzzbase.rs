use std::{
    env::temp_dir,
    error::Error,
    fmt::Display,
    fs,
    path::Path,
    process::{Command, Output, Stdio},
};

use crate::{debug, fuzz::feature_list::FEATURES};

#[derive(Debug)]
pub enum FResult {
    CompileSuccess(Output),
    CompileError(Output),
    InternalCompileError(Output),
    RunSuccess,
    RunError,
}
impl Display for FResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileSuccess(_) => write!(f, "Compile Success :)"),
            Self::CompileError(_) => write!(f, "Compile Error :|"),
            Self::InternalCompileError(_) => write!(f, "ICE?!?!?! :("),
            _ => write!(f, "Not defined"),
        }
    }
}

pub trait Fuzzer: Send + Sync {
    fn generate(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn compile(
        &self,
        code: &[u8],
        output_source: &Path,
        output_bin: &Path,
        extra_args: &[&str],
    ) -> Result<FResult, Box<dyn Error>> {
        {
            let tmp_file = temp_dir().join(output_source);
            self.dump(code, &tmp_file)?;
            let args = [
                tmp_file.to_str().unwrap().to_string(),
                "-o".to_string(),
                output_bin.to_str().unwrap().to_string(),
            ];
            let args: Vec<String> = args
                .into_iter()
                .chain(extra_args.into_iter().map(|s| s.to_string()))
                .collect();
            let extra_args: Vec<String> = FEATURES
                .iter()
                .map(|s| format!("-Zcrate-attr=feature({s})"))
                .collect();
            let args: Vec<String> = args.into_iter().chain(extra_args.into_iter()).collect();
            let mut cmd = Command::new("rustc");
            cmd.args(args);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            let status = cmd.output()?;
            debug!("Code: {}", status.status.code().unwrap());
            if status.status.success() {
                return Ok(FResult::CompileSuccess(status));
            }
            if status.status.code().unwrap() == 1 {
                return Ok(FResult::CompileError(status));
            }
            Ok(FResult::InternalCompileError(status))
        }
    }
    fn dump(&self, code: &[u8], output: &Path) -> Result<(), Box<dyn Error>> {
        fs::write(output, code)?;
        Ok(())
    }
    fn run(&mut self, bin_path: &Path) -> Result<Output, Box<dyn Error>> {
        let mut cmd = Command::new(bin_path);
        let status = cmd.output()?;
        Ok(status)
    }
}
