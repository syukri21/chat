use crate::{SharedDebugState, WebModule};
use axum::{response::Html, Extension, Json};
use fakers::Faker;
use shaku_axum::Inject;

pub async fn get_activate_link(
    Extension(debug_state): Extension<SharedDebugState>,
) -> Json<String> {
    let x = debug_state.read().await.token.clone();
    let string = format!("{:?}", x);
    Json::from(string)
}

pub async fn create_dummy_user(faker_service: Inject<WebModule, dyn Faker>) -> Html<String> {
    faker_service.generate_random_users(100).await.unwrap();
    Html("ok".to_string())
}
