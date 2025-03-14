use std::str::FromStr;

use axum::{
    response::{Html, IntoResponse, Response},
    Form,
};
use axum_extra::extract::Multipart;
use commons::generic_errors::GenericError;
use http::StatusCode;
use jwt::AccessClaims;
use serde::Deserialize;
use shaku_axum::Inject;
use tracing::error;
use usecases::userdetail_usecase::UserDetailUsecase;
use user_details::entity::UserDetail;
use uuid::Uuid;

use crate::WebModule;

#[derive(Deserialize)]
pub struct UpdateProfileForm {
    first_name: String,
    last_name: String,
    dob: Option<String>,
    gender: Option<String>,
    profile_picture: Option<String>,
}

impl UpdateProfileForm {
    pub fn to_user_detail(&self, user_id: Uuid) -> anyhow::Result<UserDetail> {
        // this date should look like this: "2000-01-01"
        let date_of_birth = self.dob.clone();
        let date_of_birth = match date_of_birth {
            Some(date) => {
                let date = chrono::NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d");
                if date.is_err() {
                    return Err(GenericError::invalid_input(String::from(
                        "Invalid date of birth",
                    )));
                }
                Some(date.unwrap())
            }
            None => None,
        };

        Ok(UserDetail {
            id: Uuid::new_v4(),
            user_id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth,
            gender: self.gender.clone(),
            profile_picture: self.profile_picture.clone(),
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
        })
    }
}

pub async fn update_profile(
    claim: axum::extract::Extension<AccessClaims>,
    user_detail_service: Inject<WebModule, dyn UserDetailUsecase>,
    Form(form): Form<UpdateProfileForm>,
) -> impl IntoResponse {
    let user_id = claim.user_id.as_str();
    let user_detail = form.to_user_detail(Uuid::from_str(user_id).unwrap());
    if user_detail.is_err() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(String::from("Failed to parse user detail"))
            .unwrap()
            .into_response();
        //return render_error_alert(user_detail.unwrap_err());
    }
    let user_detail = user_detail.unwrap();
    match user_detail_service.update_profile(&user_detail).await {
        Ok(_) => Html(include_str!("../../page/htmx/success_update_profile.html")).into_response(),
        Err(e) => {
            error!("Error occurred during update_profile: {}", e);
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(String::from("sd"))
                .unwrap()
                .into_response()
        }
    }
}

pub async fn upload_profile_picture(
    claim: axum::extract::Extension<AccessClaims>,
    user_detail_service: Inject<WebModule, dyn UserDetailUsecase>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Iterate over the multipart fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap_or_default().to_string();
        // If this is the file field, read it into a Vec<u8>
        if field_name == "profile_picture" {
            let file_data = field.bytes().await.unwrap();
            let result = user_detail_service
                .upload_profile_picture(&claim.user_id, &file_data)
                .await;

            if result.is_err() {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(String::from("Failed to upload profile picture"))
                    .unwrap()
                    .into_response();
            }

            return Html(include_str!("../../page/htmx/success_update_profile.html"))
                .into_response();
        }
    }

    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(String::from("No file found"))
        .unwrap()
        .into_response()
}
