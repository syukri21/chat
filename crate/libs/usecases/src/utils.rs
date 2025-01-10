use persistence::{DatabaseInterface, Env, DB};
use std::sync::Arc;

#[allow(dead_code)]
pub async fn setup_db() -> Arc<dyn DatabaseInterface + Send + Sync> {
    let db_path = ":memory:"; // Use an in-memory database for tests
    let env = Env {
        db_url: format!("sqlite:{}", db_path),
        email_from: "".to_string(),
        email_from_email: "".to_string(),
        email_smtp_username: "".to_string(),
        email_smtp_password: "".to_string(),
        email_smtp_host: "".to_string(),
        email_smtp_port: "".to_string(),
        app_key_main: "".to_string(),
        app_callback_url: "".to_string(),
        app_key_jwt: "".to_string(),
    };
    let db = Arc::new(DB::new(env).await.unwrap());

    // Apply migrations
    let pool = db.get_pool();
    sqlx::migrate!("../../../migrations")
        .run(&*pool)
        .await
        .expect("Failed to run database migrations");

    db
}
