mod c_cpp;
pub mod css;
mod java;
mod python;
mod execution_summary;

pub use execution_summary::*;

use std::env;
use std::path::PathBuf;

use anyhow::Context;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::database::model::{Template, TestCase};
use crate::enums::ProgrammingLanguage;

fn random_directory() -> PathBuf {
    loop {
        let mut path = env::temp_dir();
        path.push(Uuid::new_v4().to_string());
        if !path.exists() {
            return path;
        }
    }
}

fn random_file_path(language: ProgrammingLanguage) -> PathBuf {
    loop {
        let mut path = random_directory();
        path.push(Uuid::new_v4().to_string());
        path.set_extension(language.get_extension());
        if !path.exists() {
            return path;
        }
    }
}

async fn write_to_random_file(
    code: &str,
    language: ProgrammingLanguage,
) -> anyhow::Result<PathBuf> {
    let code_path = random_file_path(language);
    fs::create_dir_all(code_path.parent().unwrap()).await?;
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&code_path)
        .await?;
    file.write_all(code.as_bytes()).await?;

    Ok(code_path)
}

#[tracing::instrument(err)]
pub async fn score(
    language: ProgrammingLanguage,
    code: &str,
    testcases: Option<Vec<TestCase>>,
    template: Option<Template>,
    question_score: u32,
) -> anyhow::Result<ExecutionResult> {
    if language == ProgrammingLanguage::Css {
        let template =
            template.context("Template is required for frontend programming language(s)")?;
        return css::execute(code, template).await;
    }

    let testcases =
        testcases.context("Testcases are required for backend programming language(s)")?;

    if language == ProgrammingLanguage::C_CPP {
        return c_cpp::execute(code, testcases, question_score).await;
    }
    if language == ProgrammingLanguage::Python {
        return python::execute(code, testcases, question_score).await;
    }
    if language == ProgrammingLanguage::Java {
        return java::execute(code, testcases, question_score).await;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::fs;

    use super::*;

    const QUESTION_SCORE: u32 = 1;

    #[rstest]
    #[trace]
    #[tokio::test]
    async fn backend(
        #[values(
            ProgrammingLanguage::Python,
            ProgrammingLanguage::C_CPP,
            ProgrammingLanguage::Java
        )]
        language: ProgrammingLanguage,
        #[files("test_data/scoring/**")] problem_path: PathBuf,
    ) {
        let mut code_path = problem_path.clone();
        code_path.push("code");
        code_path.set_extension(language.get_extension());
        let code = String::from_utf8(fs::read(code_path).unwrap()).unwrap();

        let mut testcases_path = problem_path;
        testcases_path.push("testcases.txt");
        let testcases_raw = String::from_utf8(fs::read(testcases_path).unwrap()).unwrap();
        let testcases = testcases_raw
            .split("\n\n")
            .array_chunks()
            .map(|[input, output]| TestCase {
                input: input.to_string(),
                output: output.to_string(),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let result = score(
            language,
            code.as_str(),
            Some(testcases),
            None,
            QUESTION_SCORE,
        )
        .await
        .unwrap();
        if let ExecutionResult::Succeed { score, runtime: _ } = result {
            assert!(score == QUESTION_SCORE)
        }
        assert!(false)
    }

    #[rstest]
    #[trace]
    #[tokio::test]
    async fn frontend(
        #[values(ProgrammingLanguage::Css)] language: ProgrammingLanguage,
        #[files("test_data/css_scoring/eye-of-sauron")] problem_path: PathBuf,
    ) -> anyhow::Result<()> {
        use image::io::Reader as ImageReader;
        use image::DynamicImage;
        use std::io::Cursor;

        let mut html_path = problem_path.clone();
        html_path.push("source");
        html_path.set_extension("html");
        let html = String::from_utf8(fs::read(html_path).unwrap()).unwrap();

        let mut template_path: PathBuf = problem_path;
        template_path.push("template.png");

        let template: DynamicImage = ImageReader::open(template_path)?.decode()?;
        let mut buffer = Vec::new();
        template.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)?;

        let percent: f32 = match css::render_diff_image(&buffer, html).await? {
            (match_percent, _) => match_percent,
        };

        assert!(percent > 90.0);
        Ok(())
    }
}
