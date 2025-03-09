use std::{env::temp_dir, error::Error, path::PathBuf, process::Command};

#[derive(Debug)]
pub enum FResult {
    CompileSuccess(String),
    CompileError(String),
    InternalCompileError(String),
    RunSuccess,
    RunError,
}

pub trait Fuzzer {
    fn replace(&mut self) -> Result<(), Box<dyn Error>>;
    fn compile(
        &mut self,
        output: &PathBuf,
        extra_args: &[String],
    ) -> Result<FResult, Box<dyn Error>> {
        {
            let tmp_file = temp_dir().join("fuzzmid.rs");
            self.dump(&tmp_file)?;
            let args = vec![
                tmp_file.to_str().unwrap().to_string(),
                "-o".to_string(),
                output.to_str().unwrap().to_string(),
            ];
            let args: Vec<String> = extra_args
                .iter()
                .chain(args.iter())
                .map(|x| x.clone())
                .collect();
            let mut cmd = Command::new("rustc");
            cmd.args(args);
            let status = cmd.output()?;
            if status.status.success() {
                return Ok(FResult::CompileSuccess(String::from_utf8(status.stdout)?));
            }
            let status_stderr = String::from_utf8(status.stderr)?;
            if status_stderr.contains("internal compiler error") {
                return Ok(FResult::InternalCompileError(status_stderr));
            }
            Ok(FResult::CompileError(status_stderr))
        }
    }
    fn dump(&mut self, output: &PathBuf) -> Result<(), Box<dyn Error>>;
}
