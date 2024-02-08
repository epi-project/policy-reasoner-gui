use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::routing::{delete, get, post};
use axum::Router;
use humanlog::{DebugMode, HumanLogger};
use policy_reasoner_client_backend::auth::{getKey, get_authenticate, logout, post_authenticate, AppState};
use policy_reasoner_client_backend::conv::post_conv;
use policy_reasoner_client_backend::deliberation::{post_access_data, post_exec_task, post_validate_workflow};
use policy_reasoner_client_backend::policy::{
    delete_deactivate_policy, get_active_policy, get_policies, get_policy, post_activate_policy, post_add_policy,
};
use policy_reasoner_client_backend::reasoner_conn::get_reasoner_connector_info;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize logging
    if std::env::var("HUMANLOGGER").is_ok() {
        if let Err(err) = HumanLogger::terminal(DebugMode::Debug).init() {
            eprintln!("WARNING: Failed to setup logger: {err} (no logging enabled for this session)");
        }
    } else {
        // initialize tracing
        tracing_subscriber::fmt::init();
    }

    // inir CORS
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        // allow requests from any origin
        .allow_origin("http://localhost:1234".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE]);

    let key = getKey("./key");

    let state = AppState { key };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/conv", post(post_conv))
        .route("/authenticate", post(post_authenticate))
        .route("/authenticate", get(get_authenticate))
        .route("/authenticate", delete(logout))
        .route("/policies", get(get_policies))
        .route("/policies", post(post_add_policy))
        .route("/policies/active", get(get_active_policy))
        .route("/policies/active", post(post_activate_policy))
        .route("/policies/active", delete(delete_deactivate_policy))
        .route("/policies/:version", get(get_policy))
        .route("/reasoner-connector-info", get(get_reasoner_connector_info))
        .route("/deliberation/task", post(post_exec_task))
        .route("/deliberation/data", post(post_access_data))
        .route("/deliberation/workflow", post(post_validate_workflow))
        .layer(cors)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
