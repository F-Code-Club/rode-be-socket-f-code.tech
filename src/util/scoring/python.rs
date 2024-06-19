use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::str;
use std::time::Instant;

use crate::database::model::TestCase;
use crate::enums::ProgrammingLanguage;
use crate::util;

use super::RuntimeError;
use super::{write_to_random_file, ExecutionResult};

fn execute_one(
    executable_path: &Path,
    testcase: &TestCase,
) -> anyhow::Result<Result<(bool, u32), RuntimeError>> {
    let start = Instant::now();

    // Run the code
    // Command: python $executable_path
    let mut process = Command::new("python")
        .arg(executable_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
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

    let is_matched = String::from_utf8(output.stdout).unwrap().trim() == testcase.output.trim();
    let run_time = (end - start).as_millis() as u32;

    Ok(Ok((is_matched, run_time)))
}

pub async fn execute(
    code: &str,
    testcases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    let path = write_to_random_file(code, ProgrammingLanguage::Python).await?;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result_raw = testcases
            .par_iter()
            .map(|testcase| execute_one(&path, testcase))
            .collect::<anyhow::Result<Result<Vec<_>, RuntimeError>>>();

        let _ = send.send(execution_result_raw);
    });
    let execution_result_raw = match recv.await?? {
        Ok(value) => value,
        Err(runtime_error) => return Ok(ExecutionResult::RuntimeError(runtime_error)),
    };
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
