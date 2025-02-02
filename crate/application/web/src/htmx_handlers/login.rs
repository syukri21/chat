use crate::utils::render_error_alert;
use crate::WebModule;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Form;
use axum_client_ip::{SecureClientIp};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use commons::generic_errors::GenericError;
use http::header::SET_COOKIE;
use serde::Deserialize;
use shaku_axum::Inject;
use tracing::log::{error, info};
use usecases::{LoginRequest, LoginUseCaseInterface};

#[derive(Deserialize)]
pub struct LoginForm {
    username_or_email: String,
    password: String,
}

impl LoginForm {
    fn to_login_request<'a>(
        &'a self,
        user_agent: &'a str,
        ip_address: &'a str,
    ) -> LoginRequest<'a> {
        LoginRequest {
            username: self.username_or_email.as_str(),
            password: self.password.as_str(),
            user_agent,
            ip_address,
        }
    }
}

pub async fn login(
    user_agent: Option<TypedHeader<UserAgent>>,
    SecureClientIp(ip): SecureClientIp,
    login_usecase: Inject<WebModule, dyn LoginUseCaseInterface>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let user_agent = user_agent
        .map(|user_agent| user_agent.0.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let ip = ip.to_string();
    info!(
        "Login started for username or email {}, user_agent: {}, ip: {}",
        form.username_or_email, user_agent, ip
    );
    let login_request = form.to_login_request(user_agent.as_str(), ip.as_str());

    match login_usecase.login(login_request).await {
        Ok(response) => {
            info!("Login successful");
            let mut headers = HeaderMap::new();
            // set cookie
            let cookie = format!("token={}; httpOnly; path=/", response.token)
                .parse()
                .unwrap();
            headers.insert(SET_COOKIE, cookie);
            headers.insert("hx-redirect", "/".parse().unwrap());

            (headers, "").into_response()
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
