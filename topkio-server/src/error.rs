use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use topkio_core::error::TopkioError;

pub struct TopkioErrorResponse(pub StatusCode, pub String);

impl TopkioErrorResponse {
    pub fn new(status: StatusCode, message: String) -> Self {
        TopkioErrorResponse(status, message)
    }
}

impl From<TopkioError> for TopkioErrorResponse {
    fn from(err: TopkioError) -> Self {
        TopkioErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl IntoResponse for TopkioErrorResponse {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}