use std::str::FromStr;

use crate::commons::{
    response_builder::{error_builder, ok_builder},
    templates::JinjaTemplate,
};
use crate::WebModule;
use axum::{extract::Query, response::IntoResponse, Extension, Json};
use jwt::AccessClaims;
use shaku_axum::Inject;
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
                .into_iter()
                .map(|user| template.htmx_user_info(user.id.to_string().as_str(), Box::new(user)))
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
    invite_private_chat_usecase
        .invite_private_chat(&usecases::InvitePrivateChatRequest {
            user_id: user_id.unwrap(),
            user_email_or_username: payload.user_email_or_username,
        })
        .await
        .map_err(|e| error_builder(e, "invite_private_chat_usecase"))
        .map(|val| {
            let user_info = Box::new(val.friend_user_info);
            let friend_id = val.friend_id.to_string();
            let htmx_chat_header = template.htmx_chat_header(&friend_id, user_info);
            let htmx_chat_box = template.htmx_chat_box(&val.chat_messages);
            ok_builder([htmx_chat_box, htmx_chat_header].join(""))
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
        .map(|user_info: usecases::userdetail_usecase::UserInfo| {
            let user_info = Box::new(user_info);
            ok_builder(template.htmx_chat_header(payload.user_id.as_str(), user_info))
        })
}
