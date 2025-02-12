use axum::{
    extract::Query,
    response::{IntoResponse, Response},
};
use commons::generic_errors::GenericError;
use http::StatusCode;
use shaku_axum::Inject;
use tracing::error;
use usecases::InvitePrivateChatUsecaseInterface;

use crate::{commons::templates::JinjaTemplate, utils::render_error_alert, WebModule};

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
            Response::builder()
                .status(StatusCode::OK)
                .body(response.join(""))
                .unwrap()
                .into_response()
        }
        Err(e) => {
            error!("Error occurred during find_user_info_list: {}", e);
            let error_message = match e.downcast_ref::<GenericError>() {
                Some(generic_error) => generic_error.to_string(),
                None => "An error occurred during find_user_info_list.".to_string(),
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
