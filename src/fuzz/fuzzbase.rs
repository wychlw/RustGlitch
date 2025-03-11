use std::{
    env::temp_dir, error::Error, fmt::Display, path::Path, process::{Command, Output, Stdio}, sync::LazyLock
};

use crate::util::gen_alnum;

#[derive(Debug)]
pub enum FResult {
    CompileSuccess(String),
    CompileError(String),
    InternalCompileError(String),
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

pub trait Fuzzer {
    fn replace(&mut self) -> Result<(), Box<dyn Error>>;
    fn generate(&mut self) -> Result<(), Box<dyn Error>>;
    fn compile(&mut self, output: &Path, extra_args: &[String]) -> Result<FResult, Box<dyn Error>> {
        {
            static TMP_FILE: LazyLock<String> = LazyLock::new(|| format!("fuzzmid_{}.rs", gen_alnum(4)));
            let tmp_file = temp_dir().join(TMP_FILE.as_str());
            self.dump(&tmp_file)?;
            let args = [
                tmp_file.to_str().unwrap().to_string(),
                "-o".to_string(),
                output.to_str().unwrap().to_string(),
            ];
            let args: Vec<String> = extra_args.iter().chain(args.iter()).cloned().collect();
            let mut cmd = Command::new("rustc");
            cmd.args(args);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            let status = cmd.output()?;
            if status.status.success() {
                return Ok(FResult::CompileSuccess(String::from_utf8(status.stdout)?));
            }
            let status_stderr = String::from_utf8(status.stderr)?;
            if status.status.code().unwrap() == 1 {
                return Ok(FResult::CompileError(status_stderr));
            }
            Ok(FResult::InternalCompileError(status_stderr))
        }
    }
    fn dump(&mut self, output: &Path) -> Result<(), Box<dyn Error>>;
    fn run(&mut self, bin_path: &Path) -> Result<Output, Box<dyn Error>> {
        let mut cmd = Command::new(bin_path);
        let status = cmd.output()?;
        Ok(status)
    }
}
