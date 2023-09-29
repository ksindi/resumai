use std::path::PathBuf;

use anyhow::Result;
use aws_sdk_textract::{primitives::Blob, types::Document};
use clap::{arg, command, Parser};
use llm_chain::{
    chains::map_reduce::Chain, executor, options, parameters, prompt, step::Step, Parameters,
};
use llm_chain_openai::chatgpt::Model;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Document filepath
    #[arg(long, env)]
    pub filepath: PathBuf,
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

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Args = Args::parse();
    let document_bytes = std::fs::read(args.filepath).expect("file exists and is readable");

    // base64 encode the document
    // let document_bytes = base64::encode(document_bytes);

    let shared_config = aws_config::from_env().load().await;
    let client = aws_sdk_textract::Client::new(&shared_config);

    let res = client
        .detect_document_text()
        .document(Document::builder().bytes(Blob::new(document_bytes)).build())
        .send()
        .await?;

    println!("{:#?}", res);

    Ok(())
}
