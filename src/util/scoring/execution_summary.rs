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

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum ExecuteOneDetail {
    Passed {
        test_case_id: i32,
        #[serde(skip)]
        run_time: u32,
    },
    Failed {
        test_case_id: i32,
        #[serde(skip)]
        run_time: u32,
    },
    RuntimeError {
        test_case_id: i32,
        #[serde(skip)]
        run_time: u32,
        reason: String,
    },
}

impl ExecuteOneDetail {
    fn get_run_time(&self) -> u32 {
        match self {
            ExecuteOneDetail::Passed {
                test_case_id: _,
                run_time,
            } => *run_time,
            ExecuteOneDetail::Failed {
                test_case_id: _,
                run_time,
            } => *run_time,
            ExecuteOneDetail::RuntimeError {
                test_case_id: _,
                run_time,
                reason: _,
            } => *run_time,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecutionResult {
    pub score: u32,
    pub run_time: u32,
    pub details: Vec<ExecuteOneDetail>,
}

impl ExecutionResult {
    pub fn from(details: Vec<ExecuteOneDetail>, question_score: u32) -> ExecutionResult {
        let total_run_time = details
            .iter()
            .map(ExecuteOneDetail::get_run_time)
            .sum::<u32>();
        let is_not_passed = details.iter().any(|detail| {
            !matches!(
                detail,
                ExecuteOneDetail::Passed {
                    test_case_id: _,
                    run_time: _
                }
            )
        });

        ExecutionResult {
            score: if is_not_passed { 0 } else { question_score },
            run_time: total_run_time,
            details,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "type")]
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
    fn get_score(&self) -> u32 {
        match self {
            ExecutionSummary::Executed(ExecutionResult {
                score,
                run_time: _,
                details: _,
            }) => *score,
            ExecutionSummary::CompilationError(_) => 0,
        }
    }
}
