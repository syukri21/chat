use crate::{SharedDebugState, WebModule};
use axum::{Extension, Json};
use credentials::{credential::Credential, credential_services};
use shaku_axum::Inject;
use usecases::userdetail_usecase::UserDetailUsecase;
use user_details::entity::UserDetail;
use users::user_services::UserServiceInterface;

pub async fn get_activate_link(
    Extension(debug_state): Extension<SharedDebugState>,
) -> Json<String> {
    let x = debug_state.read().await.token.clone();
    let string = format!("{:?}", x);
    Json::from(string)
}

pub async fn create_dummy_user(
    user_service: Inject<WebModule, dyn UserServiceInterface>,
    credential_services: Inject<WebModule, dyn credential_services::CredentialServiceInterface>,
    user_detail_service: Inject<WebModule, dyn UserDetailUsecase>,
) -> Json<String> {
    let user_id = uuid::Uuid::new_v4();
    let user_name = format!("{}", user_id);
    let email = format!("dummy_{}@email.com", user_id);
    let password = "dummy".to_string();

    let user = users::user::User {
        id: user_id,
        username: user_name,
        email,
        password,
        is_active: true,
        created_at: Some(chrono::Local::now().naive_local()),
        updated_at: Some(chrono::Local::now().naive_local()),
        deleted_at: None,
    };
    let _ = user_service
        .create_user(&user)
        .await
        .expect("Failed to create dummy user");

    let credential = Credential::new(user.id, "private_key", "public_key");
    let _ = credential_services
        .create_credential(&credential)
        .await
        .expect("Failed to create dummy credential");

    let mut user_detail = UserDetail::new(user.id);
    user_detail.first_name = "Jhon doe".to_string();
    user_detail.last_name = "Doe".to_string();
    let _ = user_detail_service.update_profile(&user_detail).await;

    Json::from(format!("{:?}", user))
}
