use std::{env, time::Duration};

use anyhow::{Context, Result};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;

const PUT_EXPIRES_IN: Duration = Duration::from_secs(30 * 60);

#[derive(Debug, Clone)]
pub struct S3Context {
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
}

impl S3Context {
    pub async fn new() -> Self {
        let bucket = env::var("S3_BUCKET").expect("S3_BUCKET environment variable not set.");

        let config = aws_config::load_from_env().await;

        let config_builder = aws_sdk_s3::config::Builder::from(&config);

        let client = aws_sdk_s3::Client::from_conf(config_builder.build());

        Self { client, bucket }
    }

    /// Generate presigned URL for downloading from S3.
    pub async fn object_exists(&self, key: &String) -> Result<bool> {
        let res = &self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        tracing::info!("Object exists: {:?}", res);

        Ok(res.is_ok())
    }

    /// Generate presigned URL for uploading to S3.
    pub async fn put_object_presigned_url(&self, key: &String) -> Result<String> {
        let presigned_req = &self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(PUT_EXPIRES_IN)?)
            .await?;

        Ok(presigned_req.uri().to_string())
    }

    /// Get content of S3 key.
    pub async fn get_object(&self, key: &str) -> Result<Vec<u8>> {
        let res = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to download {} from S3 bucket {}.",
                    key, &self.bucket
                )
            })?;

        let res = res.body.collect().await.unwrap().into_bytes().to_vec();

        Ok(res)
    }

    /// Create S3 object.
    pub async fn put_object(&self, key: &str, body: &[u8]) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(body.to_vec()))
            .send()
            .await
            .with_context(|| format!("Failed to upload {} to S3 bucket {}.", key, &self.bucket))?;

        Ok(())
    }
}
