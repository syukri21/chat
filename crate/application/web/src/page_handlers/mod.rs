use axum::extract::{self, Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use commons::generic_errors::GenericError;
use jwt::AccessClaims;
use minijinja::context;
use shaku_axum::Inject;
use tracing::log::info;
use usecases::userdetail_usecase::UserDetailUsecase;
use usecases::RegisterUseCaseInterface;

use crate::commons::templates::JinjaTemplate;
use crate::WebModule;

pub async fn chat(
    user_detail_usecase: Inject<WebModule, dyn UserDetailUsecase>,
    template: Inject<WebModule, dyn crate::commons::templates::JinjaTemplate>,
    claim: extract::Extension<AccessClaims>,
) -> impl IntoResponse {
    let chat_page = template.env().get_template("chat").unwrap();
    if let Ok(user_info) = user_detail_usecase.get_user_info(&claim.user_id).await {
        let profile_picture = user_info
            .user_details
            .and_then(|details| details.profile_picture)
            .unwrap_or_else(|| {
                format!(
                    "https://ui-avatars.com/api/?name={}&background=random&rounded=true",
                    user_info.username
                )
            });
        let render = chat_page
            .render(context! {
                profile_picture => profile_picture
            })
            .unwrap();
        return Html(render);
    }
    let sww_page = template
        .env()
        .get_template("something-went-wrong")
        .unwrap()
        .render(context! {
            title => "500 - Internal Server Error<"
        })
        .unwrap();
    Html(sww_page)
}

pub async fn login() -> Html<&'static str> {
    Html(include_str!("../../page/login.html"))
}
pub async fn signup() -> Html<&'static str> {
    Html(include_str!("../../page/signup.html"))
}

pub async fn profile(
    user_detail_usecase: Inject<WebModule, dyn UserDetailUsecase>,
    template: Inject<WebModule, dyn JinjaTemplate>,
    claim: extract::Extension<AccessClaims>,
) -> Html<String> {
    let Ok(user_info) = user_detail_usecase.get_user_info(&claim.user_id).await else {
        return Html(template.something_went_wrong_page());
    };

    //<img id="profile-preview" src="https://ui-avatars.com/api/?name={{username}}" alt="Profile Picture"

    let profile_picture = format!(
        "https://ui-avatars.com/api/?name={}&background=random&rounded=true",
        &user_info.username
    );

    let template = template.env().get_template("profile").unwrap();

    match user_info.user_details {
        Some(user_detail) => {
            let profile_picture = &user_detail
                .profile_picture
                .map_or_else(|| profile_picture, |x| x.to_string());

            let dob = &user_detail
                .date_of_birth
                .map_or_else(String::new, |date_of_birth| date_of_birth.to_string());

            let gender = &user_detail
                .gender
                .map_or_else(|| "Male".to_string(), |gender| gender.to_string());

            let template = template
                .render(context! {
                    username =>  &user_info.username,
                    email =>  &user_info.email,
                    profile_picture =>  profile_picture.as_str(),
                    last_name => &user_detail.last_name,
                    first_name=> &user_detail.first_name,
                    dob=> dob,
                    gender=> gender,
                })
                .unwrap();
            Html(template)
        }
        _ => {
            let template = template
                .render(context! {
                    username =>  &user_info.username,
                    email =>  &user_info.email,
                    profile_picture =>  profile_picture.as_str(),
                    last_name => "",
                    first_name=> "",
                    dob=> "",
                    gender=> "",
                })
                .unwrap();
            Html(template)
        }
    }
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
