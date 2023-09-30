use std::path::PathBuf;

use anyhow::Result;
use clap::{arg, command, Parser};
use llm_chain::{executor, options, parameters, prompt};
use llm_chain_openai::chatgpt::Model;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Document filepath
    #[arg(long, env)]
    pub filepath: PathBuf,
}

/// Analyze resume.
async fn analyze_resume(resume_text: &String) -> Result<()> {
    let opts = options!(Model: Model::Gpt4);
    let exec = executor!(chatgpt, opts)?;
    let res = prompt!(
r#"
You are a model trained to analyze resumes to identify specific key attributes and provide a detailed analysis. Please analyze the following resume text and provide commentary and a score for each attribute listed below. The text is parsed from a PDF resume and should be treated with mindfulness for various formats and potential parsing issues.
"#,
r#"

Resume Text:
`{{text}}`

### Analysis:

#### 1. **Title Progression:**
- **Objective:** Identify if the candidate has shown progression in their titles indicating career growth and not stagnancy.
- **Commentary:**

#### 2. **Tenures:**
- **Objective:** Ensure the candidate has tenures of at least 2 years at each company they have worked for.
- **Commentary:**

#### 3. **Achievements:**
- **Objective:** Look for achievements that indicate a positive impact on the business and handling of large-scale operations (e.g., requests per second) using data.
- **Commentary:**

#### 4. **Experience as a Software Engineer:**
- **Objective:** Verify that the candidate has at least 5 years of experience as a software engineer.
- **Commentary:**

#### 5. **Leadership in Projects:**
- **Objective:** Confirm that the candidate has led projects.
- **Commentary:**

#### 6. **Knowledge of DevOps, AWS, and Infrastructure:**
- **Objective:** Check for keywords such as DevOps, AWS, Infrastructure, and CI/CD to ensure the candidate has knowledge in these areas.
- **Commentary:**

### Overall Scores:
Provide an overall score for each attribute on a scale of 1-10 and a final cumulative score.

1. **Title Progression:** [Score]
2. **Tenures:** [Score]
3. **Achievements:** [Score]
4. **Experience as a Software Engineer:** [Score]
5. **Leadership in Projects:** [Score]
6. **Knowledge of DevOps, AWS, and Infrastructure:** [Score]

### Final Cumulative Score: [Total Score]
"#
)
    .run(&parameters!(resume_text), &exec)
    .await?;

    println!("{}", res.to_immediate().await?.as_content());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Args = Args::parse();

    let bytes = std::fs::read(args.filepath)?;
    let text = pdf_extract::extract_text_from_mem(&bytes).expect("Failed to extract text from PDF");

    println!("Resume text: {}", text);

    analyze_resume(&text).await?;

    Ok(())
}
