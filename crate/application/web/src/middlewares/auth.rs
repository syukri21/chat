use std::sync::Arc;

use axum::{
    extract::{self, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use http::StatusCode;
use tracing::{error, trace};
use usecases::LoginUseCaseInterface;

use crate::commons::constants::{DEBUG_PAGES, PUBLIC_PAGES};

fn check_path(current_path: &str) -> bool {
    let is_path = |path: &&str| {
        if path.contains("*") {
            let path = path.replace("*", "");
            return current_path.starts_with(&path);
        };
        *path == current_path
    };
    let is_public = PUBLIC_PAGES.iter().any(is_path);
    let is_debug = DEBUG_PAGES.iter().any(is_path);
    is_debug || is_public
}

pub async fn auth(
    State(login_usecase): State<Arc<dyn LoginUseCaseInterface>>,
    cookie_jar: CookieJar,
    mut req: extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let current_path = req.uri().path();

    if check_path(current_path) {
        return Ok(next.run(req).await);
    }

    let auth_header = cookie_jar
        .get("token")
        .ok_or_else(|| {
            error!("No auth token for");
            StatusCode::UNAUTHORIZED
        })
        .map_err(|e1| {
            error!("No auth token {}", e1);
            StatusCode::UNAUTHORIZED
        })?
        .value();

    trace!("Auth header: {}", auth_header);
    let claims = login_usecase
        .authorize_current_user(auth_header)
        .await
        .map_err(|e| {
            error!("Error when authorizing current user: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
