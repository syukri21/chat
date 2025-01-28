use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use commons::generic_errors::GenericError;
use std::sync::Arc;
use tracing::log::info;
use usecases::RegisterUseCaseInterface;

pub async fn home() -> Html<&'static str> {
    Html(include_str!("../../page/chat.html"))
}
pub async fn login() -> Html<&'static str> {
    Html(include_str!("../../page/login.html"))
}
pub async fn signup() -> Html<&'static str> {
    Html(include_str!("../../page/signup.html"))
}
pub async fn callback_activate(
    State(register_usecase): State<Arc<dyn RegisterUseCaseInterface>>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    info!("Activating user");
    match register_usecase.activate_user(token.as_str()).await {
        Ok(_) => {
            info!("Activation successful");
            Html(
                include_str!("../../page/callback/activate.html")
                    .parse::<String>()
                    .unwrap(),
            )
            .into_response()
        }
        Err(e) => {
            tracing::error!("Activation user failed: {}", e);
            let error_message = match e.downcast_ref::<GenericError>() {
                Some(generic_error) => generic_error.to_string(),
                None => "An error during activation user".to_string(),
            };
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(error_message)
                .unwrap()
                .into_response()
        }
    }
}
