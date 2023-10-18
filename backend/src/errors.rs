use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    pub message: String,
    #[serde(skip)]
    pub status: StatusCode,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        tracing::error!("Internal server error: {}", error);

        AppError {
            message: "Internal server error".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
