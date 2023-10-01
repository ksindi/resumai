use axum::{handler, Router};
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

async fn generate_presigned_url() -> String {
    let s3_client = S3Client::new(Region::UsWest1);
    let req = PutObjectRequest {
        bucket: "your-bucket-name".to_string(),
        key: "your-object-key".to_string(),
        ..Default::default()
    };
    let presigned_url = s3_client.get_presigned_url(req, 3600).await.unwrap();
    presigned_url
}

async fn presigned_url_endpoint() -> impl Into<axum::response::IntoResponse> {
    let presigned_url = generate_presigned_url().await;
    axum::response::Json(presigned_url)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/presigned-url", handler::get(presigned_url_endpoint))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
