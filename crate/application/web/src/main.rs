use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::{HeaderMap, Request};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{middleware, Router};
use axum_client_ip::SecureClientIpSource;
use chats::chat_services::ChatService;
use commons::templates::{JinjaTemplate, JinjaTemplateImpl};
use crate::htmx_handlers::{login, register};
use credentials::credential_services::CredentialService;
use crypto::Crypto;
use fakers::{FakerImpl, FakerInnerImpl};
use htmx_handlers::{chat, user_detail};
use jwt::JWT;
use log::{error, info};
use mail::Mail;
use middlewares::auth::auth;
use persistence::{Env, DB};
use sessions::services::SessionService;
use shaku::{module, HasComponent};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::RwLock;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{debug, info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use usecases::userdetail_usecase::UserDetailUsecaseImpl;
use usecases::{InvitePrivateChatUsecase, LoginUseCase, LoginUseCaseInterface, RegisterUseCase};
use user_details::user_detail_service::UserDetailServiceImpl;
use users::user_services::UserService;

// Add this near the top with other modules
mod commons;
mod debug_handlers;
mod htmx_handlers;
mod middlewares;
mod page_handlers;
mod utils;

module! {
     WebModule {
        components = [
            ChatService,
            CredentialService,
            Crypto ,
            DB,
            Env,
            FakerImpl,
            FakerInnerImpl,
            InvitePrivateChatUsecase,
            JWT,
            JinjaTemplateImpl,
            LoginUseCase,
            Mail,
            RegisterUseCase,
            SessionService,
            UserDetailServiceImpl,
            UserDetailUsecaseImpl,
            UserService,
        ],

        providers = []
    }
}

#[derive(Default, Debug)]
pub struct DebugState {
    pub token: HashMap<String, String>,
}

type SharedDebugState = Arc<RwLock<DebugState>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_init();
    let env = Env::load();
    let module_builder = WebModule::builder()
        .with_component_override::<dyn JinjaTemplate>(Box::new(JinjaTemplateImpl::default()));
    let module = usecases::utils::setup_module::<WebModule>(module_builder, env).await;
    let login_usecase: Arc<dyn LoginUseCaseInterface> = module.resolve();
    let arc_module = Arc::new(module);
    let debug_state = Arc::new(RwLock::new(DebugState {
        token: HashMap::new(),
    }));

    // build our application with a route
    // In main function
    let htmx_app = Router::new()
        .route("/register", post(register::register))
        .route("/find-users", get(chat::find_user_info_list))
        .route(
            "/invite-private-chat",
            post(chat::invite_private_chat_usecase),
        )
        .route("/update-profile", post(user_detail::update_profile))
        .route(
            "/upload-profile-picture",
            post(user_detail::upload_profile_picture),
        )
        .route("/login", post(login::login));

    // This is callback nest routes
    let callback_app =
        Router::new().route("/activate/{token}", get(page_handlers::callback_activate));

    let debug_app = Router::new()
        .route("/create-dummy-user", get(debug_handlers::create_dummy_user))
        .route("/active-link", get(debug_handlers::get_activate_link));

    let app = Router::new()
        .route("/", get(page_handlers::chat))
        .route("/login", get(page_handlers::login))
        .route("/signup", get(page_handlers::signup))
        .route("/profile", get(page_handlers::profile));

    let app = app
        .nest("/htmx", htmx_app)
        .nest("/callback", callback_app)
        .nest("/debug", debug_app)
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(AddExtensionLayer::new(debug_state))
        .route_layer(middleware::from_fn_with_state(login_usecase.clone(), auth))
        .with_state(arc_module);

    let app = with_assets(app);
    let app = with_tracing(app);

    // run it
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

fn tracing_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,user_details=debug,tower_http=debug,axum::rejection=trace,mail=debug,commons=debug,sessions=debug,usecases=info",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
fn with_assets(router: Router) -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let index_assets = assets_dir.join("index.html");
    let dir = ServeDir::new(assets_dir).not_found_service(ServeFile::new(index_assets));
    router
        .nest_service("/assets", dir.clone())
        .fallback_service(dir)
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
                info!("request");
            })
            .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                info!("response completed in {:?}", latency);
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
                |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    error!("request failed with error {}", error);
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
