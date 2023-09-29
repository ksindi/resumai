use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use clap::{arg, command, Parser};
use llm_chain::{
    chains::map_reduce::Chain, executor, options, parameters, prompt, step::Step, Parameters,
};
use llm_chain_openai::chatgpt::Model;
use reqwest::header;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gong API Access Key
    #[arg(long, env)]
    pub gong_api_access_key: String,

    /// Gong API Access Key Secret
    #[arg(long, env)]
    pub gong_api_access_key_secret: String,

    /// Gong API Base Path
    #[arg(long, env)]
    pub gong_api_base_path: String,

    /// Name of the company for summarization
    #[arg(long, env = "GONG_SUMMARIZER_COMPANY_NAME")]
    pub company_name: String,
}

/// Split a transcript into pages of a maximum word count so that we don't go over API token limits.
pub fn split_transcript_into_pages(transcript: &CallTranscript, max_words: usize) -> Vec<String> {
    let mut pages = Vec::new();
    let mut current_page = String::new();
    let mut current_word_count = 0;

    if let Some(ref monologues) = transcript.transcript {
        for monologue in monologues {
            let unknown = "Unknown".to_string();
            let empty_vec = vec![];
            let speaker_id = monologue.speaker_id.as_ref().unwrap_or(&unknown);
            let sentences = monologue.sentences.as_ref().unwrap_or(&empty_vec);

            let combined_sentences: String = sentences
                .iter()
                .filter_map(|sentence| sentence.text.as_ref())
                .cloned()
                .collect::<Vec<String>>()
                .join(" ");

            let word_count = combined_sentences.split_whitespace().count();
            assert!(
                word_count <= max_words,
                "Sentence word count exceeds max words."
            );

            if current_word_count + word_count > max_words {
                pages.push(current_page.clone());
                current_page.clear();
                current_word_count = 0;
            }

            current_page.push_str(&format!("{}: {}\n\n", speaker_id, combined_sentences));
            current_word_count += word_count;
        }
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

/// Summarize a list of pages into a single summary.
async fn summarize_pages(
    pages: Vec<String>,
    company_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let opts = options!(Model: Model::Gpt4);
    let exec = executor!(chatgpt, opts)?;
    let map_prompt = create_map_prompt(&company_name);
    let reduce_prompt = create_reduce_prompt();
    let chain = Chain::new(map_prompt, reduce_prompt);
    let docs = pages.into_iter().map(|page| parameters!(page)).collect();

    let res = chain.run(docs, Parameters::new(), &exec).await?;
    tracing::info!("{}", res.to_immediate().await?.as_content());

    Ok(())
}

/// Create a prompt for the map step.
fn create_map_prompt(company_name: &str) -> Step {
    Step::for_prompt_template(prompt!(
        &format!(
            r#"
"You are a bot designed to summarize call transcripts between {} and its customers. Your main goal is to help product managers and engineers quickly understand the key pain points customers face. Transcripts are formatted as:
`SpeakerId: Paragraph`.

Given a transcript, extract and list the customer's main issues, referencing key phrases from the call to back up each finding. 
"#,
            company_name
        ),
        r#"

Task: Summarize the following transcript into bullet points that highlight customer pain points with their current solution:
`{{text}}`"
"#
    ))
}

/// Create a prompt for the reduce step.
fn create_reduce_prompt() -> Step {
    Step::for_prompt_template(prompt!(
        "You are a diligent bot that summarizes text",
        "Please combine the articles below into one summary as bullet points:\n{{text}}"
    ))
}

/// Create a header map for Gong API authentication.
fn create_auth_headers(api_key: &str, api_secret: &str) -> Result<header::HeaderMap> {
    let basic_token: String =
        general_purpose::STANDARD.encode(format!("{}:{}", api_key, api_secret));
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&format!("Basic {}", basic_token))?;
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);
    Ok(headers)
}

/// Create a Gong API configuration.
fn create_gong_config(client: reqwest::Client, gong_api_base_path: &str) -> Configuration {
    Configuration {
        base_path: gong_api_base_path.to_owned(),
        client,
        user_agent: Some("Gong Summarizer".to_owned()),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();
    let headers = create_auth_headers(&args.gong_api_access_key, &args.gong_api_access_key_secret)?;
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let config = create_gong_config(client, &args.gong_api_base_path);

    let calls_filter = CallsFilter {
        from_date_time: Some("2023-09-01T00:00:00Z".to_owned()),
        to_date_time: Some("2024-01-01T00:00:00Z".to_owned()),
        workspace_id: None,
        call_ids: None,
    };

    let api_filter = PublicApiBaseRequestV2CallsFilter {
        cursor: None,
        filter: Box::new(calls_filter),
    };

    let transcripts = get_call_transcripts(&config, api_filter).await?;

    // TODO update this
    if let Some(transcript) = transcripts
        .call_transcripts
        .as_ref()
        .and_then(|vec| vec.first())
        .cloned()
    {
        let pages = split_transcript_into_pages(&transcript, 2000);

        tracing::info!("Transcript pages: {}", pages.len());

        match summarize_pages(pages, &args.company_name).await {
            Ok(_) => tracing::info!("Summary successful!"),
            Err(e) => tracing::info!("Error summarizing: {}", e),
        }
    } else {
        tracing::info!("No transcript found.");
    }

    Ok(())
}
