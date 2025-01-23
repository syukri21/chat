use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Form;
use commons::generic_errors::GenericError;
use serde::Deserialize;
use std::sync::Arc;
use usecases::{RegisterRequest, RegisterUseCaseInterface};

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
    email: String,
    private_key: String,
    public_key: String,
}

impl RegisterForm {
    pub fn to_register_request(&self) -> RegisterRequest {
        RegisterRequest {
            username: &self.username,
            email: &self.email,
            password: &self.password,
            private_key: &self.private_key,
            public_key: &self.public_key,
        }
    }
}
fn render_error_alert(message: String) -> String {
    format!(
        r#"<div class="mb-4 p-4 text-red-700 bg-red-100 rounded-lg" role="alert">
            <p>{}</p>
        </div>"#,
        message,
    )
}

pub async fn register(
    State(register_usecase): State<Arc<dyn RegisterUseCaseInterface>>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    tracing::info!("Htmx register Started with username: {}", form.username);

    match register_usecase.register(&form.to_register_request()).await {
        Ok(_) => Html(
            include_str!("../page/htmx/signup_success.html")
                .parse::<String>()
                .unwrap(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Registration failed: {}", e);
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
