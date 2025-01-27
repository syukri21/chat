use crate::utils::render_error_alert;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Form;
use commons::generic_errors::GenericError;
use serde::Deserialize;
use std::sync::Arc;
use tracing::log::{error, info, trace};
use usecases::{LoginRequest, LoginUseCaseInterface};

#[derive(Deserialize)]
pub struct LoginForm {
    username_or_email: String,
    password: String,
}

impl LoginForm {
    fn to_login_request(&self) -> LoginRequest {
        LoginRequest {
            username: self.username_or_email.as_str(),
            password: self.password.as_str(),
        }
    }
}

pub async fn login(
    State(login_usecase): State<Arc<dyn LoginUseCaseInterface>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    info!(
        "Login started for username or email {}",
        form.username_or_email
    );
    match login_usecase.login(form.to_login_request()).await {
        Ok(response) => {
            trace!("login response: {:?}", response);
            info!("Login successful");

            let mut headers = HeaderMap::new();
            headers.insert("hx-redirect", "/".parse().unwrap());
            (headers, "It works!").into_response()
        }
        Err(e) => {
            error!("Error occurred during login: {}", e);
            let error_message = match e.downcast_ref::<GenericError>() {
                Some(generic_error) => generic_error.to_string(),
                None => "An error occurred during registration.".to_string(),
            };
            let error_html = render_error_alert(error_message);
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(error_html)
                .unwrap()
                .into_response()
        }
    }
}
