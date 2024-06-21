use std::fmt::Display;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, thiserror::Error)]
pub struct CompilationError {
    pub reason: String,
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, ToSchema)]
pub enum DetailKind {
    Passed,
    Failed,
    RuntimeError,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Detail {
    pub test_case_id: i32,
    #[serde(skip)]
    pub run_time: u32,
    pub runtime_error: Option<String>,
    pub kind: DetailKind,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum ResultKind {
    CompilationError,
    Executed,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecutionResult {
    pub score: u32,
    pub run_time: u32,
    pub details: Option<Vec<Detail>>,
    pub compilation_error: Option<String>,
    pub kind: ResultKind
}

impl ExecutionResult {
    pub fn from_details(details: Vec<Detail>, question_score: u32) -> ExecutionResult {
        let total_run_time = details
            .iter()
            .fold(0, |total_run_time, detail| total_run_time + detail.run_time);
        let is_not_passed = details
            .iter()
            .any(|detail| detail.kind != DetailKind::Passed);

        ExecutionResult {
            score: if is_not_passed { 0 } else { question_score },
            run_time: total_run_time,
            details: Some(details),
            compilation_error: None,
            kind: ResultKind::Executed
        }
    }

    pub fn from_compilation_error(error: CompilationError) -> ExecutionResult {
        ExecutionResult {
            score: 0,
            run_time: 0,
            details: None,
            compilation_error: Some(error.reason),
            kind: ResultKind::CompilationError
        }
    }
}