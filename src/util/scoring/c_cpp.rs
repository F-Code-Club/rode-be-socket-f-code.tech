use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;
use std::time::Instant;

use crate::database::model::TestCase;
use crate::enums::ProgrammingLanguage;
use crate::util;

use super::write_to_random_file;
use super::CompilationError;
use super::ExecutionResult;
use super::RuntimeError;

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

fn execute_one(
    executable_path: &Path,
    testcase: &TestCase,
) -> anyhow::Result<Result<(bool, u32), RuntimeError>> {
    let start = Instant::now();

    // Run the code
    // Command: $executable_path
    let mut process = Command::new(executable_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write input to stdin
    let mut stdin = process.stdin.take().unwrap();
    stdin.write_all(testcase.input.as_bytes())?;
    drop(stdin);

    let runtime_error = util::process::capture_stderr(&mut process)?;

    let output = process.wait_with_output()?;

    let end = Instant::now();

    if !runtime_error.is_empty() {
        return Ok(Err(RuntimeError));
    }

    let is_matched = String::from_utf8(output.stdout)?.trim() == testcase.output.trim();
    let run_time = (end - start).as_millis() as u32;

    Ok(Ok((is_matched, run_time)))
}

pub async fn execute(
    code: &str,
    testcases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    let code_path = write_to_random_file(code, ProgrammingLanguage::C_CPP).await?;

    let executable_path = compile(code_path).await??;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result_raw = testcases
            .par_iter()
            .map(|testcase| execute_one(&executable_path, testcase))
            .collect::<anyhow::Result<Result<Vec<_>, RuntimeError>>>();

        let _ = send.send(execution_result_raw);
    });
    let execution_result_raw = recv.await???;
    let (is_all_matched, total_run_time) =
        execution_result_raw
            .into_iter()
            .fold((true, 0), |acc, current| {
                let is_matched = acc.0 && current.0;
                let run_time = acc.1 + current.1;

                (is_matched, run_time)
            });

    Ok(ExecutionResult::Succeed {
        score: if is_all_matched { question_score } else { 0 },
        runtime: total_run_time,
    })
}
