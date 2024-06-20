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
    pub reason: Option<String>,
    pub kind: DetailKind,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecutionResult {
    pub score: u32,
    pub run_time: u32,
    pub details: Vec<Detail>,
}

impl ExecutionResult {
    pub fn from(details: Vec<Detail>, question_score: u32) -> ExecutionResult {
        let total_run_time = details
            .iter()
            .fold(0, |total_run_time, detail| total_run_time + detail.run_time);
        let is_not_passed = details
            .iter()
            .any(|detail| detail.kind != DetailKind::Passed);

        ExecutionResult {
            score: if is_not_passed { 0 } else { question_score },
            run_time: total_run_time,
            details,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub enum ExecutionSummary {
    CompilationError(CompilationError),
    Executed(ExecutionResult),
}

impl From<CompilationError> for ExecutionSummary {
    fn from(value: CompilationError) -> Self {
        ExecutionSummary::CompilationError(value)
    }
}

impl From<ExecutionResult> for ExecutionSummary {
    fn from(value: ExecutionResult) -> Self {
        ExecutionSummary::Executed(value)
    }
}

impl ExecutionSummary {
    // Return score and total run time
    pub fn get_metrics(&self) -> (u32, u32) {
        match self {
            ExecutionSummary::Executed(ExecutionResult {
                score,
                run_time,
                details: _,
            }) => (*score, *run_time),
            ExecutionSummary::CompilationError(_) => (0, 0),
        }
    }
}
