use std::str::FromStr;

use crate::commons::templates::JinjaTemplate;
use crate::{utils::render_error_alert, WebModule};
use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Extension, Json,
};
use commons::generic_errors::GenericError;
use http::StatusCode;
use jwt::AccessClaims;
use minijinja::context;
use shaku_axum::Inject;
use tracing::error;
use usecases::userdetail_usecase::UserDetailUsecase;
use usecases::InvitePrivateChatUsecaseInterface;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FindUserRequest {
    pub search_friend: String,
}

pub async fn find_user_info_list(
    invite_private_chat_usecase: Inject<WebModule, dyn InvitePrivateChatUsecaseInterface>,
    template: Inject<WebModule, dyn JinjaTemplate>,
    Query(query): Query<FindUserRequest>,
) -> impl IntoResponse {
    match invite_private_chat_usecase
        .find_user_info_list(query.search_friend.as_str())
        .await
    {
        Ok(response) => {
            let response: Vec<String> = response
                .iter()
                .map(|user| template.htmx_user_info(user))
                .collect();
            ok_builder(response.join(""))
        }
        Err(e) => error_builder(e, "find_user_info_list"),
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct InvitePrivateChatRequest {
    pub user_email_or_username: String,
}

pub async fn invite_private_chat_usecase(
    invite_private_chat_usecase: Inject<WebModule, dyn InvitePrivateChatUsecaseInterface>,
    claim: Extension<AccessClaims>,
    template: Inject<WebModule, dyn JinjaTemplate>,
    Json(payload): Json<InvitePrivateChatRequest>,
) -> impl IntoResponse {
    let user_id = Uuid::from_str(&claim.user_id);
    let chat_window = template
        .env()
        .get_template("htmx-chat-window")
        .unwrap()
        .render(context! {})
        .unwrap();

    invite_private_chat_usecase
        .invite_private_chat(&usecases::InvitePrivateChatRequest {
            user_id: user_id.unwrap(),
            user_email_or_username: payload.user_email_or_username,
        })
        .await
        .map_err(|e| error_builder(e, "invite_private_chat_usecase"))
        .map(|val| {
            let htmx_chat_header =
                template.htmx_chat_header(val.friend_id.to_string().as_str(), val.friend_user_info);
            ok_builder([chat_window, htmx_chat_header].join(""))
        })
}

#[derive(serde::Deserialize)]
pub struct ChatHeaderRequest {
    pub user_id: String,
}

pub async fn chat_header(
    user_detail_usecase: Inject<WebModule, dyn UserDetailUsecase>,
    template: Inject<WebModule, dyn JinjaTemplate>,
    Query(payload): Query<ChatHeaderRequest>,
) -> impl IntoResponse {
    user_detail_usecase
        .get_user_info(payload.user_id.as_str())
        .await
        .map_err(|e| error_builder(e, "invite_private_chat_usecase"))
        .map(|user_info| ok_builder(template.htmx_chat_header(payload.user_id.as_str(), user_info)))
}

fn error_builder(e: anyhow::Error, key: &str) -> http::Response<axum::body::Body> {
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

fn ok_builder(response: String) -> http::Response<axum::body::Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(response)
        .unwrap()
        .into_response()
}
