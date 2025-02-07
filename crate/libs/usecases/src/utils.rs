use persistence::db::database::DBParameters;
use persistence::db::sqlite::create_sqlite_db_pool;
use persistence::env::myenv::EnvInterface;
use persistence::{DatabaseInterface, Env, DB};
use shaku::{HasComponent, ModuleBuilder};
use std::sync::Arc;

#[allow(dead_code)]
pub async fn setup_db() -> Arc<dyn DatabaseInterface> {
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

pub async fn setup_module<
    T: shaku::Module + HasComponent<(dyn EnvInterface)> + HasComponent<(dyn DatabaseInterface)>,
>(
    module_builder: ModuleBuilder<T>,
    env: Env,
) -> T {
    let pool = Arc::new(create_sqlite_db_pool(env.get_db_url()).await.unwrap());
    
    module_builder
        .with_component_parameters::<DB>(DBParameters {
            pool: Some(pool.clone()),
        })
        .with_component_override::<dyn EnvInterface>(Box::new(env))
        .build()
}
