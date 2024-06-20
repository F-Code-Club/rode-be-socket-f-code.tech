use std::{io::Write as _, path::Path, process::Stdio, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{database::model::TestCase, enums::ProgrammingLanguage, util};

use super::{
    create_unique_project, CompilationError, ExecuteOneDetail, ExecutionResult, ExecutionSummary,
};

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

fn execute_one(
    project_path: &Path,
    main_file_name: &str,
    language: ProgrammingLanguage,
    test_case: &TestCase,
) -> anyhow::Result<ExecuteOneDetail> {
    let test_case_id = test_case.id;

    let start = Instant::now();

    // Run the code
    let mut process = get_execute_command(main_file_name, language)
        .current_dir(project_path)
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
    main_file_name: &'static str,
    language: ProgrammingLanguage,
    code: &str,
    test_cases: Vec<TestCase>,
    question_score: u32,
) -> anyhow::Result<ExecutionSummary> {
    let project_path = create_unique_project(code, language).await?;

    if let Err(compilation_error) = compile(&project_path, main_file_name, language).await? {
        return Ok(ExecutionSummary::CompilationError(compilation_error));
    }

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let execution_result_raw = test_cases
            .par_iter()
            .map(|test_case| execute_one(&project_path, main_file_name, language, test_case))
            .collect::<anyhow::Result<Vec<_>>>()
            .map(|details| ExecutionResult::from(details, question_score));

        let _ = send.send(execution_result_raw);
    });
    let execution_result = recv.await??;

    Ok(ExecutionSummary::Executed(execution_result))
}
