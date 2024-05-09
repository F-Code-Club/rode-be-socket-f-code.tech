use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::str;
use std::time::Instant;

use crate::database::model::Testcase;
use crate::enums::ProgrammingLanguage;

use super::write_to_random_file;
use super::ExecutionResult;

async fn compile(code_path: &Path) -> anyhow::Result<()> {
    // Command: javac $code_path
    tokio::process::Command::new("javac")
        .arg(code_path)
        .spawn()?
        .wait()
        .await?;

    Ok(())
}

fn execute_one(project_path: &Path, testcase: &Testcase) -> (bool, u32) {
    let start = Instant::now();

    // Run the code
    // Command: java -c -p $executable_path Main
    let mut process = Command::new("java")
        .current_dir(project_path)
        .arg("Main")
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
    testcases: Vec<Testcase>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    let code_path = write_to_random_file(code, ProgrammingLanguage::Java).await?;
    let project_path = code_path.parent().unwrap().to_path_buf();

    compile(&code_path).await?;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let (is_all_matched, total_run_time) = testcases
            .par_iter()
            .map(|testcase| execute_one(&project_path, testcase))
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
