use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    body::{Body, Bytes},
    extract::Path,
    http::{header, Request, StatusCode},
    middleware::{self as axum_middleware, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use lambda_runtime::tower::ServiceBuilder;
use serde_json::json;
use tower_http::trace::TraceLayer;

mod evaluate;
mod logging;
mod s3;

async fn buffer_and_print<B>(
    direction: &str,
    path: String,
    body: B,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!(direction, ?path, body);
    }

    Ok(bytes)
}

/// Print request and response information
async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = req.uri().path().to_owned();
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", path.clone(), body).await?;
    let query = parts.uri.query().unwrap_or_default();
    tracing::info!("query string: {query}");
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", path, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;

    res.headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
    );
    res
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Invalid endpoint")
}

async fn app() -> Router {
    // We might implement middleware logic like checking for headers that's not needed for status
    // so we will create a separate router for it
    let status_router: Router = Router::new().route("/status", get(|| async { "OK" }));

    Router::new()
        .route("/upload", post(upload))
        .route(
            "/evaluations/:evaluation_id",
            get(get_evaluation).delete(delete_resume_and_evaluation),
        )
        .route(
            "/evaluations/:evaluation_id/download_resume",
            get(download_resume),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(axum_middleware::from_fn(print_request_response))
                .layer(axum_middleware::from_fn(propagate_header)),
        )
        //.route_layer(axum_middleware::from_fn(authorization_check))
        .merge(status_router)
        .fallback(handler_404)
}

async fn upload() -> Result<impl IntoResponse, (StatusCode, String)> {
    let s3_context = s3::S3Context::new().await;

    let evaluation_id = uuid::Uuid::new_v4().to_string();

    let s3_key = format!("resumes/{}", evaluation_id);

    let upload_url = s3_context
        .put_object_presigned_url(&s3_key)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get presigned url: {err}"),
            )
        })?;

    let result = json!({
        "upload_url": upload_url,
        "evaluation_id": evaluation_id,
    });

    Ok((StatusCode::ACCEPTED, Json(result)).into_response())
}

async fn delete_resume_and_evaluation(
    Path(evaluation_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let s3_context = s3::S3Context::new().await;

    let resume_key = format!("resumes/{}", evaluation_id.to_string());
    let evaluation_key = format!("results/{}", evaluation_id.to_string());

    tracing::info!("Checking if object exists: {}", resume_key);

    let resume_exists = s3_context.object_exists(&resume_key).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to check resume status: {err}"),
        )
    })?;

    if !resume_exists {
        return Err((
            StatusCode::NOT_FOUND,
            format!("resume {} not found", evaluation_id),
        ));
    }

    tracing::info!("Checking if object exists: {}", evaluation_key);

    let evaluation_exists = s3_context
        .object_exists(&evaluation_key)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to check evaluation status: {err}"),
            )
        })?;

    if !evaluation_exists {
        return Err((
            StatusCode::NOT_FOUND,
            format!("evaluation {} not found", evaluation_id),
        ));
    }

    s3_context.delete_object(&resume_key).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to delete resume: {err}"),
        )
    })?;

    s3_context
        .delete_object(&evaluation_key)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to delete evaluation: {err}"),
            )
        })?;

    Ok((StatusCode::ACCEPTED, "OK").into_response())
}

async fn get_evaluation(
    Path(evaluation_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let s3_context = s3::S3Context::new().await;

    let key = format!("results/{}", evaluation_id.to_string());

    tracing::info!("Checking if object exists: {}", key);

    let object_exists = s3_context.object_exists(&key).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to check evaluation status: {err}"),
        )
    })?;

    if !object_exists {
        return Err((
            StatusCode::NOT_FOUND,
            format!("evaluation {} not found", evaluation_id),
        ));
    }

    let evaluation = s3_context.get_object(&key).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to get evaluation: {err}"),
        )
    })?;

    let evaluation = String::from_utf8(evaluation).unwrap();

    Ok((StatusCode::ACCEPTED, Json(evaluation)).into_response())
}

async fn download_resume(
    Path(evaluation_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let s3_context = s3::S3Context::new().await;

    let key = format!("resumes/{}", evaluation_id.to_string());

    tracing::info!("Checking if object exists: {}", key);

    let object_exists = s3_context.object_exists(&key).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to check resume status: {err}"),
        )
    })?;

    if !object_exists {
        return Err((
            StatusCode::NOT_FOUND,
            format!("resume {} not found", evaluation_id),
        ));
    }

    let filename = format!("resume-{}.pdf", evaluation_id);

    let presigned_url = s3_context
        .get_object_presigned_url(&key, &filename)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get presigned url: {err}"),
            )
        })?;

    let result = json!({
        "download_url": presigned_url,
    });

    Ok((StatusCode::ACCEPTED, Json(result)).into_response())
}

#[tokio::main]
async fn main() {
    logging::setup_logging();

    // We need to set the port to 8080 for the AWS lambda adapter layer to work
    let addr = std::env::var("SOCKET_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&socket_addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}
