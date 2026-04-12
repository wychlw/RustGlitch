use std::error::Error;

use clap::ValueEnum;

use crate::conf::FuzzerType;
use crate::pipeline::types::{BuiltinStage, JobType, StageKind};

#[derive(Clone, Debug)]
pub struct FuzzerJob {
    pub tasker: String,
    pub fuzzer: Option<FuzzerType>,
    pub task_name: String,
    pub stage: StageKind,
    pub task_arg: Option<String>,
}

impl FuzzerJob {
    pub fn parser(s: &str) -> Result<FuzzerJob, Box<dyn Error + Send + Sync>> {
        let (tasker_raw, task_expr) = s
            .split_once(':')
            .ok_or("Missing task separator ':'")?;
        let tasker = tasker_raw.trim().to_string();

        let task_expr = task_expr.trim();

        let (task_name, task_arg) = if let Some(p) = task_expr.find('(') {
            let end = task_expr.rfind(')').ok_or("Missing ')' in task args")?;
            if end <= p {
                return Err("Invalid task args format".into());
            }
            let name = task_expr[..p].trim().to_string();
            let arg = task_expr[p + 1..end].trim().to_string();
            let arg = if arg.is_empty() { None } else { Some(arg) };
            (name, arg)
        } else if let Some((name, arg)) = task_expr.split_once(':') {
            let name = name.trim().to_string();
            let arg = arg.trim().to_string();
            let arg = if arg.is_empty() { None } else { Some(arg) };
            (name, arg)
        } else {
            (task_expr.trim().to_string(), None)
        };

        let mut task_name = task_name;
        let mut task_arg = task_arg;

        let stage = if tasker.eq_ignore_ascii_case("filter") {
            if !task_name.eq_ignore_ascii_case("filter") {
                task_arg = Some(task_expr.to_string());
            }
            task_name = "filter".to_string();
            StageKind::Builtin(BuiltinStage::Filter)
        } else if tasker.eq_ignore_ascii_case("dump") {
            if task_name.eq_ignore_ascii_case("raw") || task_name.eq_ignore_ascii_case("pretty") {
                StageKind::Builtin(BuiltinStage::Dump)
            } else {
                if !task_name.eq_ignore_ascii_case("dump") {
                    task_arg = Some(task_expr.to_string());
                }
                task_name = "dump".to_string();
                StageKind::Builtin(BuiltinStage::Dump)
            }
        } else if task_name.eq_ignore_ascii_case("filter") {
            StageKind::Builtin(BuiltinStage::Filter)
        } else if task_name.eq_ignore_ascii_case("dump") {
            StageKind::Builtin(BuiltinStage::Dump)
        } else {
            let job_type = JobType::from_str(&task_name, true).map_err(|_| "Invalid job type")?;
            StageKind::Job(job_type)
        };

        let fuzzer_type = FuzzerType::from_str(&tasker, true).ok();

        if matches!(stage, StageKind::Job(_)) && fuzzer_type.is_none() {
            return Err("This task requires a valid tasker/fuzzer type".into());
        }

        Ok(FuzzerJob {
            tasker,
            fuzzer: fuzzer_type,
            task_name,
            stage,
            task_arg,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{BuiltinStage, StageKind};

    #[test]
    fn parse_simple_stage() {
        let j = FuzzerJob::parser("rustc:fuzz").unwrap();
        assert_eq!(j.tasker, "rustc");
        assert_eq!(j.task_name, "fuzz");
        assert!(j.task_arg.is_none());
    }

    #[test]
    fn parse_paren_args_stage() {
        let j = FuzzerJob::parser("gate:filter(compile-error)").unwrap();
        assert_eq!(j.tasker, "gate");
        assert_eq!(j.task_name, "filter");
        assert_eq!(j.task_arg.as_deref(), Some("compile-error"));
    }

    #[test]
    fn parse_colon_args_stage() {
        let j = FuzzerJob::parser("gate:filter:compile-error").unwrap();
        assert_eq!(j.tasker, "gate");
        assert_eq!(j.task_name, "filter");
        assert_eq!(j.task_arg.as_deref(), Some("compile-error"));
    }

    #[test]
    fn parse_filter_short_stage() {
        let j = FuzzerJob::parser("filter:ice+success").unwrap();
        assert_eq!(j.tasker, "filter");
        assert_eq!(j.task_name, "filter");
        assert_eq!(j.task_arg.as_deref(), Some("ice+success"));
        assert_eq!(j.stage, StageKind::Builtin(BuiltinStage::Filter));
    }

    #[test]
    fn parse_dump_short_stage() {
        let j = FuzzerJob::parser("dump:raw").unwrap();
        assert_eq!(j.tasker, "dump");
        assert_eq!(j.task_name, "raw");
        assert!(j.task_arg.is_none());
        assert_eq!(j.stage, StageKind::Builtin(BuiltinStage::Dump));
    }

    #[test]
    fn parse_dump_legacy_stage() {
        let j = FuzzerJob::parser("dump:dump:pretty").unwrap();
        assert_eq!(j.tasker, "dump");
        assert_eq!(j.task_name, "dump");
        assert_eq!(j.task_arg.as_deref(), Some("pretty"));
        assert_eq!(j.stage, StageKind::Builtin(BuiltinStage::Dump));
    }
}
