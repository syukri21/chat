use persistence::{DatabaseInterface, Env, DB};
use std::sync::Arc;

#[allow(dead_code)]
pub(crate) async fn setup() -> Arc<dyn DatabaseInterface + Send + Sync> {
    let db_path = ":memory:"; // Use an in-memory database for tests
    let env = Env {
        db_url: format!("sqlite:{}", db_path),
    };
    let db = Arc::new(DB::new(env).await.unwrap());

    // Apply migrations
    let pool = db.get_pool();
    sqlx::migrate!("../migrations")
        .run(&*pool)
        .await
        .expect("Failed to run database migrations");

    db
}
