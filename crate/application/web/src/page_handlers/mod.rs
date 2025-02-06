use crate::WebModule;
use axum::extract::{self, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use commons::generic_errors::GenericError;
use jwt::AccessClaims;
use shaku_axum::Inject;
use tracing::log::info;
use usecases::userdetail_usecase::UserDetailUsecase;
use usecases::RegisterUseCaseInterface;

const SOMETHING_WENT_WRONG: &str = include_str!("../../page/500.html");

pub async fn home() -> Html<&'static str> {
    Html(include_str!("../../page/chat.html"))
}
pub async fn login() -> Html<&'static str> {
    Html(include_str!("../../page/login.html"))
}
pub async fn signup() -> Html<&'static str> {
    Html(include_str!("../../page/signup.html"))
}

const PROFILE_TEMPLATE: &str = include_str!("../../page/profile.html");
pub async fn profile(
    user_detail_usecase: Inject<WebModule, dyn UserDetailUsecase>,
    claim: extract::Extension<AccessClaims>,
) -> Html<String> {
    let Ok(user_info) = user_detail_usecase.get_user_info(&claim.user_id).await else {
        return Html(SOMETHING_WENT_WRONG.to_string());
    };

    //<img id="profile-preview" src="https://ui-avatars.com/api/?name={{username}}" alt="Profile Picture"

    let profile_picture = format!(
        "https://ui-avatars.com/api/?name={}&background=random&rounded=true",
        &user_info.username
    );

    let template = PROFILE_TEMPLATE
        .replace("{{username}}", &user_info.username)
        .replace("{{email}}", &user_info.email);

    match user_info.user_details {
        Some(user_detail) => {
            let template = template
                .replace("{{last_name}}", &user_detail.last_name)
                .replace("{{first_name}}", &user_detail.first_name)
                .replace(
                    "{{profile_picture}}",
                    &user_detail
                        .profile_picture
                        .map_or_else(|| profile_picture, |x| x.to_string()),
                )
                .replace(
                    "{{dob}}",
                    &user_detail
                        .date_of_birth
                        .map_or_else(String::new, |date_of_birth| date_of_birth.to_string()),
                )
                .replace(
                    "{{gender}}",
                    &user_detail
                        .gender
                        .map_or_else(|| "Male".to_string(), |gender| gender.to_string()),
                );
            return Html(template);
        }
        _ => {
            let template = template
                .replace("{{profile_picture}}", profile_picture.as_str())
                .replace("{{last_name}}", "")
                .replace("{{first_name}}", "")
                .replace("{{dob}}", "")
                .replace("{{gender}}", "");
            return Html(template);
        }
    };
}

pub async fn callback_activate(
    register_usecase: Inject<WebModule, dyn RegisterUseCaseInterface>,
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
