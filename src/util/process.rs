use std::io::BufRead;

use anyhow::Context;
use tokio::io::AsyncBufReadExt as _;

pub fn capture_stderr(stderr: std::process::ChildStderr) -> anyhow::Result<String> {
    let mut error = String::new();

    let lines = std::io::BufReader::new(stderr).lines();
    for line in lines {
        let line = line?;
        error.push_str(&line);
        error.push('\n');
    }
    error.pop();

    Ok(error)
}

pub fn capture_stdout(stdout: std::process::ChildStdout) -> anyhow::Result<String> {
    let mut output = String::new();

    let lines = std::io::BufReader::new(stdout).lines();
    for line in lines {
        let line = line?;
        output.push_str(&line);
        output.push('\n');
    }
    output.pop();

    Ok(output)
}

pub async fn capture_stderr_async(process: &mut tokio::process::Child) -> anyhow::Result<String> {
    let stderr = process.stderr.take().context("Failed to take the stderr of the process")?;

    let mut error = String::new();

    let mut lines = tokio::io::BufReader::new(stderr).lines();
    while let Some(line) = lines.next_line().await? {
        error.push_str(&line);
        error.push('\n');
    }
    error.pop();

    Ok(error)
}
