use axum::response::{IntoResponse, Response};
use http::StatusCode;
use log::error;

use crate::utils::render_error_alert;

use commons::generic_errors::GenericError;

pub fn error_builder(e: anyhow::Error, key: &str) -> http::Response<axum::body::Body> {
    error!("Error occurred during {}: {}", key, e);
    let error_message = match e.downcast_ref::<GenericError>() {
        Some(generic_error) => generic_error.to_string(),
        None => format!("An error occurred during {}.", key).to_string(),
    };
    let error_html = render_error_alert(error_message);
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(error_html)
        .unwrap()
        .into_response()
}

pub fn ok_builder(response: String) -> http::Response<axum::body::Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(response)
        .unwrap()
        .into_response()
}
