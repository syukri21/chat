use crate::SharedDebugState;
use axum::{Extension, Json};

pub async fn get_activate_link(
    Extension(debug_state): Extension<SharedDebugState>,
) -> Json<String> {
    let x = debug_state.read().await.token.clone();
    let string = format!("{:?}", x);
    Json::from(string)
}
