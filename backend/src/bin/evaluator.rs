use anyhow::Result;
use aws_lambda_events::{event::s3::S3Event, s3::S3EventRecord};
use futures::future::join_all;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use mime::APPLICATION_PDF;
use mime_sniffer::MimeTypeSniffer;
use resumai::{evaluate::analyze_resume, logging::setup_logging, s3::S3Context};

/// Check if the file is a PDF.
async fn is_allowed_mime_type(buffer: Vec<u8>) -> Result<bool> {
    let mime_type = match buffer.sniff_mime_type() {
        Some(s) => s,
        None => return Ok(false),
    }
    .parse::<mime::Mime>()
    .expect("Failed to parse MIME type");

    tracing::info!("Mime type found: {}", mime_type);

    Ok(mime_type == APPLICATION_PDF)
}

async fn process_event_record(record: S3EventRecord) -> Result<(), Error> {
    let s3_context = S3Context::new().await;

    tracing::info!("Record: {:?}", record);

    // Safely attempt to unwrap the key and handle errors
    let get_key = record
        .s3
        .object
        .key
        .ok_or_else(|| Error::from("Object key not found"))?;

    tracing::info!("Key: {}", get_key);

    // Add tracing span for key
    let _span = tracing::info_span!("key", key = get_key.as_str()).entered();

    // Handle potential errors when getting the object from S3
    let bytes = s3_context
        .get_object(get_key.as_str())
        .await
        .map_err(|e| Error::from(format!("Failed to get object from S3: {}", e)))?;

    let allowed_mime_type = is_allowed_mime_type(bytes.clone()).await?;

    if !allowed_mime_type {
        tracing::info!("Mime type not allowed. Found {:?}", bytes.sniff_mime_type());
        return Ok(());
    }

    // Handle potential errors when extracting text from PDF
    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| Error::from(format!("Failed to extract text from PDF: {}", e)))?;

    tracing::info!("Text: {}", text);

    let result = analyze_resume(&text).await?;

    tracing::info!("Result from OpenAI: {:?}", result);

    // Remove the resumes/ prefix from the key and replace with results/
    let put_key = get_key.replace("resumes/", "results/");

    tracing::info!("Put key: {}", put_key);

    // Handle potential errors when putting the object back to S3
    s3_context
        .put_object(&put_key, result.as_bytes())
        .await
        .map_err(|e| Error::from(format!("Failed to put object: {}", e)))?;

    tracing::info!("Successfully put object to S3");

    Ok(())
}

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    tracing::debug!("Event: {:?}", event);

    let process = move |record: S3EventRecord| async move { process_event_record(record).await };

    let futures = event
        .payload
        .records
        .into_iter()
        .map(process)
        .collect::<Vec<_>>();

    let _ = join_all(futures).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logging();

    tracing::info!("Starting lambda");

    run(service_fn(function_handler)).await
}
