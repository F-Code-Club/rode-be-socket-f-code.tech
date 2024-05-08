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

fn execute_one(project_path: &Path, testcase: &Testcase) -> ExecutionResult {
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

    // Get output and check
    let output = process.wait_with_output().unwrap();
    println!("{}", String::from_utf8(output.stdout.clone()).unwrap().trim());
    let is_match = String::from_utf8(output.stdout).unwrap().trim() == testcase.output.trim();

    let end = Instant::now();

    ExecutionResult {
        score: if is_match { 1u32 } else { 0u32 },
        run_time: (end - start).as_millis() as u32,
    }
}

pub async fn execute(code: &str, testcases: Vec<Testcase>) -> anyhow::Result<ExecutionResult> {
    let code_path = write_to_random_file(code, ProgrammingLanguage::Java).await?;
    let project_path = code_path.parent().unwrap().to_path_buf();

    compile(&code_path).await?;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result = testcases
            .par_iter()
            .map(|testcase| execute_one(&project_path, testcase))
            .reduce(ExecutionResult::zero, |acc, current| acc + current);

        let _ = send.send(execution_result);
    });
    let execution_result = recv.await?;

    Ok(execution_result)
}
