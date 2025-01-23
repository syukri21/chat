use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, Request};
use axum::response::{Html, Response};
use axum::routing::{get, post};
use axum::Router;
use chats::chat_services::ChatService;
use credentials::credential_services::CredentialService;
use crypto::Crypto;
use jwt::JWT;
use mail::Mail;
use persistence::{DatabaseInterface, Env, DB};
use shaku::{module, HasComponent};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use usecases::{utils, InvitePrivateChatUsecase, RegisterUseCase, RegisterUseCaseInterface};
use users::user_services::UserService;

// Add this near the top with other modules
mod htmx_handler;

module! {
     WebModule {
        components = [InvitePrivateChatUsecase, RegisterUseCase, UserService, ChatService, CredentialService, Env, DB, JWT, Mail, Crypto ],
        providers = []
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_init();

    let module = utils::setup_module::<WebModule>(WebModule::builder()).await;
    let db: &dyn DatabaseInterface = module.resolve_ref();
    db.migrate().await;

    let register_usecase: Arc<dyn RegisterUseCaseInterface> = module.resolve();

    // build our application with a route
    // In main function
    let htmx_app = Router::new().route(
        "/register",
        post(htmx_handler::register).with_state(register_usecase),
    );
    let app = Router::new()
        .route("/", get(home))
        .route("/login", get(login))
        .route("/signup", get(signup))
        .nest("/htmx", htmx_app);

    let app = with_assets(app);
    let app = with_tracing(app);

    // run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn with_assets(router: Router) -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let index_assets = assets_dir.join("index.html");
    let dir = ServeDir::new(assets_dir).not_found_service(ServeFile::new(index_assets));
    router
        .nest_service("/assets", dir.clone())
        .fallback_service(dir)
}
fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn with_tracing(app: Router) -> Router {
    // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
    // It provides good defaults but is also very customizable.
    //
    // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
    //
    // If you want to customize the behavior using closures here is how.
    app.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    trace_id = uuid::Uuid::new_v4().to_string().as_str(),
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {
                // You can use `_span.record("some_other_field", value)` in one of these
                // closures to attach a value to the initially empty field in the info_span
                // created above.
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                // ...
            })
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                // ...
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                    // ...
                },
            )
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    // ...
                },
            ),
    )
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::info!("Shutting down...");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        tracing::info!("Shutting down...");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
async fn home() -> Html<&'static str> {
    Html(include_str!("../page/chat.html"))
}
async fn login() -> Html<&'static str> {
    Html(include_str!("../page/login.html"))
}
async fn signup() -> Html<&'static str> {
    Html(include_str!("../page/signup.html"))
}
// Remove the htmx_register function as it's now in htmx_handler.rs
