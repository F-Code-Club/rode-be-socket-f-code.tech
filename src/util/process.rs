use std::io::{BufRead, Write};

use tokio::io::AsyncBufReadExt as _;

pub fn write_to_stdin(process: &mut std::process::Child, data: &[u8]) -> anyhow::Result<()> {
    let mut stdin = process.stdin.take().unwrap();
    stdin.write_all(data)?;

    Ok(())
}

pub fn capture_stderr(process: &mut std::process::Child) -> anyhow::Result<String> {
    let stderr = process.stderr.take().unwrap();

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

pub async fn capture_stderr_async(process: &mut tokio::process::Child) -> anyhow::Result<String> {
    let stderr = process.stderr.take().unwrap();

    let mut error = String::new();

    let mut lines = tokio::io::BufReader::new(stderr).lines();
    while let Some(line) = lines.next_line().await? {
        error.push_str(&line);
        error.push('\n');
    }
    error.pop();

    Ok(error)
}
