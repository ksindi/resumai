use anyhow::Result;
use llm_chain::{executor, options, parameters, prompt};
use llm_chain_openai::chatgpt::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeEvaluation {
    career: u8,
    proficiency: u8,
    impact: u8,
    communication: u8,
    innovation: u8,
    high_signal: u8,
}

#[allow(dead_code)]
pub fn extract_last_json(text: &str) -> Option<&str> {
    let last_open_brace = text.rfind('{')?;
    let last_close_brace = text.rfind('}')?;
    if last_open_brace < last_close_brace {
        Some(&text[last_open_brace..=last_close_brace])
    } else {
        None
    }
}

/// Analyze resume.
#[allow(dead_code)]
pub async fn analyze_resume(resume_text: &String) -> Result<String> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ssm::Client::new(&config);

    tracing::info!("Getting OpenAI API key");

    let openai_api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => client
            .get_parameter()
            .name("/prod/resumai/openai-key")
            .with_decryption(true)
            .send()
            .await
            .expect("Failed to get OpenAI API key")
            .parameter
            .expect("OpenAI API key not found")
            .value
            .unwrap(),
    };

    tracing::info!("Making request to OpenAI API");

    let opts = options!(Model: Model::Gpt4, ApiKey: openai_api_key);
    let exec = executor!(chatgpt, opts)?;

    let res = prompt!(
r#"
You are a model trained to analyze resumes to identify specific key attributes and provide a detailed analysis. Please analyze the following resume text and provide commentary and a score for each attribute listed below (Score from 1 to 10, where 1 is the lowest and 10 is the highest, and anything above 8 is considered exceptional). The text is parsed from a PDF resume and should be treated with mindfulness for various formats and potential parsing issues. Be a harsh grader keeping in mind the highest standards in the industry.
"#,
r#"

Resume Text:
`{{text}}`

## Analysis

### 1. **Career Trajectory:**
- **Objective:** Assess title progression, tenures, and employment at respected or venture-backed companies. Penalize if the company is not well-known (-2) or if the candidate has a history of job-hopping (-1). Also, heavily penalize if they have less than 6 years of experience as a software engineer or lack a senior title (-3).
- **Commentary:**

### 2. **Technical Proficiency:**
- **Objective:** Evaluate experience, knowledge of DevOps practices such as CI/CD, and leadership in technical projects. Penalize if they list out technologies they know without context (-3).
- **Commentary:**

### 3. **Quantifiable Impact:**
- **Objective:** Seek impactful achievements with clear, quantifiable outcomes that demonstrate the candidate's significant contributions. Penalize if there are not metrics shared (-4).
- **Commentary:**

### 4. **Professionalism, Communication, and Attention to Detail:**
- **Objective:** Ensure excellent communication skills, attention to detail, and proper grammar to ascertain the candidate's professionalism and effectiveness in communication. Penalize for any grammatical errors (-1 for each error).
- **Commentary:**

### 5. **Innovative and Distinctive Factors:**
- **Objective:** Look for signs of innovation, distinctive elements, and personal initiatives or projects. Reward for patents, published papers, or personal projects (+2).
- **Commentary:**

### 6. **High Signal Traits:**
- **Objective:** Evaluate knowledge in high-signal areas like Rust, participation in math or comp sci Olympiads, and attendance at elite universities. Assess problem-solving ability, open-source contributions, continuous learning, adaptability, and passion for technology. Consider recommendations or references. Penalize if no high signal traits are evident (-3).
- **Commentary:**

## Scores:
Provide a score for each category and a final cumulative score.

1. **Career Trajectory:** [Score]
2. **Technical Proficiency:** [Score]
3. **Quantifiable Impact:** [Score]
4. **Professionalism, Communication, and Attention to Detail:** [Score]
5. **Innovative and Distinctive Factors:** [Score]
6. **High Signal Traits:** [Score]

As part of your output, you should hide the objective in each section. This is important.

You should not assume the gender of the individual. Use pronouns they/them/their.

### Final Cumulative Score: [Total Score]

## JSON Output:
Provide the scores in a JSON format.

```json
{
    "career": [Career Trajectory Score],
    "proficiency": [Technical Proficiency Score],
    "impact": [Quantifiable Impact Score],
    "communication": [Professionalism, Communication, and Attention to Detail Score],
    "innovation": [Innovative and Distinctive Factors Score],
    "high_signal": [High Signal Traits Score]
}
```
"#
)
    .run(&parameters!(resume_text), &exec)
    .await?;

    let content = res.to_immediate().await?.as_content().to_text();

    Ok(content)
}
