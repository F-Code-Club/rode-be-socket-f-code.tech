pub mod backend;
mod execution_summary;
pub mod frontend;

pub use execution_summary::*;

use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str;
use std::time::Instant;

use anyhow::Context;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::database::model::{Template, TestCase};
use crate::enums::ProgrammingLanguage;
use crate::util;

use self::backend::c_cpp;

pub const MAIN_FILE_NAME: &str = "Main";

fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

async fn create_unique_directory(code: &str) -> anyhow::Result<PathBuf> {
    let now = util::time::now();
    let unique_id = hash((now, code)).to_string();
    let mut path = env::temp_dir();
    path.push(unique_id);
    fs::create_dir(&path).await?;
    Ok(path)
}

/// Create a unique project using the hash of time and code
pub async fn create_unique_project(
    code: &str,
    language: ProgrammingLanguage,
) -> anyhow::Result<PathBuf> {
    let project_path = create_unique_directory(code).await?;

    let mut main_file_path = project_path.clone();
    main_file_path.push(MAIN_FILE_NAME);
    main_file_path.set_extension(language.get_extension());

    let mut main_file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&main_file_path)
        .await?;

    main_file.write_all(code.as_bytes()).await?;

    Ok(project_path)
}
#[tracing::instrument(err)]
pub async fn score(
    language: ProgrammingLanguage,
    code: &str,
    test_cases: Option<Vec<TestCase>>,
    template: Option<Template>,
    question_score: u32,
) -> anyhow::Result<ExecutionSummary> {
    if language == ProgrammingLanguage::Css {
        let template =
            template.context("Template is required for frontend programming language(s)")?;
        return frontend::css::execute(code, template).await;
    }

    let test_cases =
        test_cases.context("Testcases are required for backend programming language(s)")?;
    backend::execute(MAIN_FILE_NAME, language, code, test_cases, question_score).await
}

// #[cfg(test)]
// mod tests {
//     use rstest::rstest;
//     use std::fs;
//
//     use super::*;
//
//     const QUESTION_SCORE: u32 = 1;
//
//     #[rstest]
//     #[trace]
//     #[tokio::test]
//     async fn backend(
//         #[values(
//             ProgrammingLanguage::Python,
//             ProgrammingLanguage::C_CPP,
//             ProgrammingLanguage::Java
//         )]
//         language: ProgrammingLanguage,
//         #[files("test_data/scoring/**")] problem_path: PathBuf,
//     ) {
//         let mut code_path = problem_path.clone();
//         code_path.push("code");
//         code_path.set_extension(language.get_extension());
//         let code = String::from_utf8(fs::read(code_path).unwrap()).unwrap();
//
//         let mut testcases_path = problem_path;
//         testcases_path.push("testcases.txt");
//         let testcases_raw = String::from_utf8(fs::read(testcases_path).unwrap()).unwrap();
//         let testcases = testcases_raw
//             .split("\n\n")
//             .array_chunks()
//             .map(|[input, output]| TestCase {
//                 input: input.to_string(),
//                 output: output.to_string(),
//                 ..Default::default()
//             })
//             .collect::<Vec<_>>();
//
//         let result = score(
//             language,
//             code.as_str(),
//             Some(testcases),
//             None,
//             QUESTION_SCORE,
//         )
//         .await
//         .unwrap();
//         if let ExecutionResult::Succeed { score, runtime: _ } = result {
//             assert!(score == QUESTION_SCORE)
//         }
//         assert!(false)
//     }
//
//     #[rstest]
//     #[trace]
//     #[tokio::test]
//     async fn frontend(
//         #[values(ProgrammingLanguage::Css)] language: ProgrammingLanguage,
//         #[files("test_data/css_scoring/eye-of-sauron")] problem_path: PathBuf,
//     ) -> anyhow::Result<()> {
//         use image::io::Reader as ImageReader;
//         use image::DynamicImage;
//         use std::io::Cursor;
//
//         let mut html_path = problem_path.clone();
//         html_path.push("source");
//         html_path.set_extension("html");
//         let html = String::from_utf8(fs::read(html_path).unwrap()).unwrap();
//
//         let mut template_path: PathBuf = problem_path;
//         template_path.push("template.png");
//
//         let template: DynamicImage = ImageReader::open(template_path)?.decode()?;
//         let mut buffer = Vec::new();
//         template.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;
//
//         let percent: f32 = match css::render_diff_image(&buffer, html).await? {
//             (match_percent, _) => match_percent,
//         };
//
//         assert!(percent > 90.0);
//         Ok(())
//     }
// }
