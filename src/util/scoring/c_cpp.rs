use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;
use std::time::Instant;

use crate::database::model::TestCase;
use crate::enums::ProgrammingLanguage;
use crate::util;

use super::ExecutionResult;
use super::{write_to_random_file, ExecuteOneDetail};
use super::{CompilationError, ExecutionSummary};

async fn compile(code_path: PathBuf) -> anyhow::Result<Result<PathBuf, CompilationError>> {
    let executable_path = code_path.with_extension("");

    // Command: gcc $code_path -o $executable_path
    let mut process = tokio::process::Command::new("gcc")
        .arg(code_path)
        .arg("-o")
        .arg(&executable_path)
        .stderr(Stdio::piped())
        .spawn()?;

    process.wait().await?;

    let compilation_error = util::process::capture_stderr_async(&mut process).await?;

    if !compilation_error.is_empty() {
        return Ok(Err(CompilationError {
            reason: compilation_error,
        }));
    }

    Ok(Ok(executable_path))
}

fn execute_one(executable_path: &Path, test_case: &TestCase) -> anyhow::Result<ExecuteOneDetail> {
    let test_case_id = test_case.id;

    let start = Instant::now();

    // Run the code
    // Command: $executable_path
    let mut process = Command::new(executable_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write input to stdin
    let mut stdin = process.stdin.take().unwrap();
    stdin.write_all(test_case.input.as_bytes())?;
    drop(stdin);

    let runtime_error = util::process::capture_stderr(&mut process)?;

    let output = process.wait_with_output()?;

    let end = Instant::now();

    let run_time = (end - start).as_millis() as u32;

    if !runtime_error.is_empty() {
        return Ok(ExecuteOneDetail::RuntimeError {
            test_case_id,
            run_time,
            reason: runtime_error,
        });
    }

    let is_matched = String::from_utf8(output.stdout)?.trim() == test_case.output.trim();

    if is_matched {
        return Ok(ExecuteOneDetail::Passed {
            test_case_id,
            run_time,
        });
    } else {
        return Ok(ExecuteOneDetail::Failed {
            test_case_id,
            run_time,
        });
    }
}

pub async fn execute(
    code: &str,
    test_cases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionSummary> {
    let code_path = write_to_random_file(code, ProgrammingLanguage::C_CPP).await?;

    let executable_path = match compile(code_path).await? {
        Ok(value) => value,
        Err(compilation_error) => {
            return Ok(ExecutionSummary::CompilationError(compilation_error));
        }
    };

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result_raw = test_cases
            .par_iter()
            .map(|test_case| execute_one(&executable_path, test_case))
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|details| ExecutionResult::from(details, question_score));

        let _ = send.send(execution_result_raw);
    });
    let execution_result = recv.await??;

    Ok(ExecutionSummary::Executed(execution_result))
 }
