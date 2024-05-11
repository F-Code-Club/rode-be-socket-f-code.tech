use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::str;
use std::time::Instant;

use crate::database::model::TestCase;
use crate::enums::ProgrammingLanguage;

use super::{write_to_random_file, ExecutionResult};

fn execute_one(executable_path: &Path, testcase: &TestCase) -> (bool, u32) {
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

    let output = process.wait_with_output().unwrap();

    let end = Instant::now();

    let is_matched = String::from_utf8(output.stdout).unwrap().trim() == testcase.output.trim();
    let run_time = (end - start).as_millis() as u32;

    (is_matched, run_time)
}

pub async fn execute(
    code: &str,
    testcases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    let path = write_to_random_file(code, ProgrammingLanguage::Python).await?;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let (is_all_matched, total_run_time) = testcases
            .par_iter()
            .map(|testcase| execute_one(&path, testcase))
            .reduce(
                || (true, 0),
                |acc, current| {
                    let is_matched = acc.0 && current.0;
                    let run_time = acc.1 + current.1;

                    (is_matched, run_time)
                },
            );

        let _ = send.send((is_all_matched, total_run_time));
    });
    let (is_all_matched, total_run_time) = recv.await?;

    Ok(ExecutionResult {
        score: if is_all_matched { question_score } else { 0 },
        run_time: total_run_time,
    })
}
