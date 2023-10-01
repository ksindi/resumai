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
You are a model trained to analyze resumes to identify specific key attributes and provide a detailed analysis. Please analyze the following resume text and provide commentary and a score for each attribute listed below. The text is parsed from a PDF resume and should be treated with mindfulness for various formats and potential parsing issues. Be a harsh grader nitpicking on every detail.
"#,
r#"

Resume Text:
`{{text}}`

### Analysis:

#### 1. **Career Trajectory:**
- **Objective:** Assess title progression, tenures, and employment at respected or venture-backed companies. Penalize if the company is not well-known or if the candidate has a history of job-hopping. Also penalize if they have less than 8 years of experience as a software engineer or lack a senior title.
- **Commentary:**

#### 2. **Technical Proficiency:**
- **Objective:** Evaluate experience, knowledge of DevOps practices such as CI/CD, and leadership in technical projects. Penalize if they list out technologies they know without context.
- **Commentary:**

#### 3. **Quantifiable Impact:**
- **Objective:** Seek impactful achievements with clear, quantifiable outcomes that demonstrate the candidate's significant contributions. Penalize if there are not metrics shared.
- **Commentary:**

#### 4. **Professionalism, Communication, and Attention to Detail:**
- **Objective:** Ensure excellent communication skills, attention to detail, and proper grammar to ascertain the candidate's professionalism and effectiveness in communication.
- **Commentary:**

#### 5. **Innovative and Distinctive Factors:**
- **Objective:** Look for signs of innovation, distinctive elements, and personal initiatives or projects.
- **Commentary:**

#### 6. **High Signal Traits:**
- **Objective:** Evaluate knowledge in high-signal areas like Rust, participation in math or comp sci Olympiads, and attendance at elite universities. Assess problem-solving ability, open-source contributions, continuous learning, adaptability, and passion for technology. Consider recommendations or references.
- **Commentary:**

### Scores:
Provide a score for each category and a final cumulative score.

1. **Career Trajectory:** [Score]
2. **Technical Proficiency:** [Score]
3. **Quantifiable Impact:** [Score]
4. **Professionalism, Communication, and Attention to Detail:** [Score]
5. **Innovative and Distinctive Factors:** [Score]
6. **High Signal Traits:** [Score]

### Final Cumulative Score: [Total Score]

### JSON Output:
Provide the scores in a JSON format.

{
    "career": [Career Trajectory Score],
    "proficiency": [Technical Proficiency Score],
    "impact": [Quantifiable Impact Score],
    "communication": [Professionalism, Communication, and Attention to Detail Score],
    "innovation": [Innovative and Distinctive Factors Score],
    "high_signal": [High Signal Traits Score]
}
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
