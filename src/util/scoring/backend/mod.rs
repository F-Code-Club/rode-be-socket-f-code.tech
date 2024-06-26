use std::{
    io::Write as _,
    path::Path,
    process::Stdio,
    thread,
    time::{self, Instant},
};

use anyhow::Context;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    config::{self, TEST_CASE_TIME_OUT},
    database::model::TestCase,
    enums::ProgrammingLanguage,
    util,
};

use super::{create_unique_project, CompilationError, Detail, DetailKind, ExecutionResult};

pub mod c_cpp;
pub mod java;
pub mod python;

fn get_compile_command(
    main_file_name: &str,
    language: ProgrammingLanguage,
) -> Option<tokio::process::Command> {
    match language {
        ProgrammingLanguage::C_CPP => c_cpp::get_compile_command(main_file_name),
        ProgrammingLanguage::Python => python::get_compile_command(),
        ProgrammingLanguage::Java => java::get_compile_command(main_file_name),
        ProgrammingLanguage::Css => {
            tracing::error!("Frontend language cant appear here");
            unreachable!()
        }
    }
}

fn get_execute_command(
    main_file_name: &str,
    language: ProgrammingLanguage,
) -> std::process::Command {
    match language {
        ProgrammingLanguage::C_CPP => c_cpp::get_execute_command(main_file_name),
        ProgrammingLanguage::Python => python::get_execute_command(main_file_name),
        ProgrammingLanguage::Java => java::get_execute_command(main_file_name),
        ProgrammingLanguage::Css => {
            tracing::error!("Frontend language cant appear here");
            unreachable!()
        }
    }
}

#[tracing::instrument(err)]
async fn compile(
    project_path: &Path,
    main_file_name: &str,
    language: ProgrammingLanguage,
) -> anyhow::Result<Result<(), CompilationError>> {
    let Some(mut compile_command) = get_compile_command(main_file_name, language) else {
        return Ok(Ok(()));
    };
    let mut process = compile_command
        .current_dir(project_path)
        .stderr(Stdio::piped())
        .spawn()?;

    process.wait().await?;

    let compilation_error = util::process::capture_stderr_async(&mut process).await?;

    if !compilation_error.is_empty() {
        return Ok(Err(CompilationError {
            reason: compilation_error,
        }));
    }

    Ok(Ok(()))
}

#[tracing::instrument(err)]
fn execute_one(
    project_path: &Path,
    main_file_name: &str,
    language: ProgrammingLanguage,
    test_case: &TestCase,
) -> anyhow::Result<Detail> {
    let test_case_id = test_case.id;

    let start = Instant::now();

    // Run the code
    let mut process = get_execute_command(main_file_name, language)
        .current_dir(project_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = process
        .stdin
        .take()
        .context("Failed to take the stderr of the process")?;
    let stderr = process
        .stderr
        .take()
        .context("Failed to take the stderr of the process")?;
    let stdout = process
        .stdout
        .take()
        .context("Failed to take the stderr of the process")?;

    let thread = thread::spawn(move || -> anyhow::Result<bool> {
        for _ in 0..*TEST_CASE_TIME_OUT {
            if let Ok(Some(_)) = process.try_wait() {
                return Ok(false);
            }
            thread::sleep(time::Duration::from_secs(1));
        }

        process.kill()?;

        Ok(true)
    });

    // Write input to stdin
    let mut input = test_case.input.clone();
    input.push('\n');
    stdin.write_all(input.as_bytes())?;

    let runtime_error = util::process::capture_stderr(stderr)?;
    let output = util::process::capture_stdout(stdout)?;

    let is_timed_out = thread
        .join()
        .map_err(|_| anyhow::anyhow!("Failed to join manger thread"))??;

    if is_timed_out {
        return Ok(Detail {
            test_case_id,
            run_time: *config::TEST_CASE_TIME_OUT,
            runtime_error: None,
            kind: DetailKind::TimedOut,
        });
    }

    let end = Instant::now();

    let run_time = (end - start).as_millis() as u32;

    if !runtime_error.is_empty() {
        return Ok(Detail {
            test_case_id,
            run_time,
            runtime_error: Some("Runtime error".to_string()),
            kind: DetailKind::RuntimeError,
        });
    }

    let is_matched = output.trim() == test_case.output.trim();

    if is_matched {
        Ok(Detail {
            test_case_id,
            run_time,
            runtime_error: None,
            kind: DetailKind::Passed,
        })
    } else {
        Ok(Detail {
            test_case_id,
            run_time,
            runtime_error: None,
            kind: DetailKind::Failed,
        })
    }
}

pub async fn execute(
    main_file_name: &'static str,
    language: ProgrammingLanguage,
    code: &str,
    test_cases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    let project_path = create_unique_project(code, language).await?;

    if let Err(compilation_error) = compile(&project_path, main_file_name, language).await? {
        return Ok(ExecutionResult::from_compilation_error(compilation_error));
    }

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result_raw = test_cases
            .par_iter()
            .map(|test_case| execute_one(&project_path, main_file_name, language, test_case))
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|details| ExecutionResult::from_details(details, question_score));

        let _ = send.send(execution_result_raw);
    });
    let execution_result = recv.await??;

    Ok(execution_result)
}
