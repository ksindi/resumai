use std::path::PathBuf;

use anyhow::Result;
use clap::{arg, command, Parser};

use resumai::evaluate::analyze_resume;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Document filepath
    #[arg(long, env)]
    pub filepath: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Args = Args::parse();

    let bytes = std::fs::read(args.filepath)?;
    println!("File size: {}", bytes.len());

    let text = pdf_extract::extract_text_from_mem(&bytes);
    println!("Resume text: {:?}", text);
    // let result = analyze_resume(&text).await?;

    //println!("{}", result);

    Ok(())
}
