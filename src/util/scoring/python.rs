use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use std::process::Command;
use std::str;
use std::time::Instant;
use std::{io::Write, process::Stdio};

use crate::database::model::Testcase;
use crate::enums::ProgrammingLanguage;

use super::{write_to_random_file, ExecutionResult};

fn execute_one(executable_path: &Path, testcase: &Testcase) -> ExecutionResult {
    let start = Instant::now();

    // Run the code
    // Command: python $executable_path
    let mut process = Command::new("python")
        .arg(executable_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Write input to stdin
    let mut stdin = process.stdin.take().unwrap();
    stdin.write_all(testcase.input.as_bytes()).unwrap();
    drop(stdin);

    // Get output and check
    let output = process.wait_with_output().unwrap();
    let is_match = String::from_utf8(output.stdout).unwrap().trim() == testcase.output.trim();

    let end = Instant::now();

    ExecutionResult {
        score: if is_match { 1u32 } else { 0u32 },
        run_time: (end - start).as_millis() as u32,
    }
}

pub async fn execute(code: &str, testcases: Vec<Testcase>) -> anyhow::Result<ExecutionResult> {
    let path = write_to_random_file(code, ProgrammingLanguage::Python).await?;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result = testcases
            .par_iter()
            .map(|testcase| execute_one(&path, testcase))
            .reduce(ExecutionResult::zero, |acc, current| acc + current);

        let _ = send.send(execution_result);
    });
    let execution_result = recv.await?;

    Ok(execution_result)
}
