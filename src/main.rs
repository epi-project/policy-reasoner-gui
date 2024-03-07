use std::env;

use axum::routing::{delete, get, post};
use axum::Router;
use clap::Parser;
use humanlog::{DebugMode, HumanLogger};
use log::debug;
use policy_reasoner_client_backend::auth::{get_authenticate, get_key, logout, post_authenticate, AppState};
use policy_reasoner_client_backend::conv::post_conv;
use policy_reasoner_client_backend::deliberation::{post_access_data, post_exec_task, post_validate_workflow};
use policy_reasoner_client_backend::policy::{
    delete_deactivate_policy, get_active_policy, get_policies, get_policy, post_activate_policy, post_add_policy,
};
use policy_reasoner_client_backend::reasoner_conn::get_reasoner_connector_info;
use specifications::address::Address;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

/***** ARGUMENTS *****/
/// Toplevel arguments
#[derive(Debug, Parser)]
struct Arguments {
    /// The address of the checker to connect to.
    #[clap(short, long, default_value = "http://localhost:3030", help = "The address of the checker to connect to/manage.")]
    checker_address: Address,
}

#[tokio::main]
async fn main() {
    // Parse arguments
    let args = Arguments::parse();

    // Initialize logging
    if std::env::var("HUMANLOGGER").is_ok() {
        if let Err(err) = HumanLogger::terminal(DebugMode::Debug).init() {
            eprintln!("WARNING: Failed to setup logger: {err} (no logging enabled for this session)");
        }
    } else {
        // initialize tracing
        tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    }

    let key = get_key("./key");

    let state = AppState { checker_address: args.checker_address, key };

    let static_base_path = env::var("CLIENT_FILES_PATH").unwrap_or_else(|_| "./clientbuild".into());
    debug!("Using client files stored at: {static_base_path}");

    let app = Router::new()
        .nest_service("/", ServeDir::new(&static_base_path).not_found_service(ServeFile::new(format!("{}/{}", &static_base_path, "index.html"))))
        .route("/api/conv", post(post_conv))
        .route("/api/authenticate", post(post_authenticate))
        .route("/api/authenticate", get(get_authenticate))
        .route("/api/authenticate", delete(logout))
        .route("/api/policies", get(get_policies))
        .route("/api/policies", post(post_add_policy))
        .route("/api/policies/active", get(get_active_policy))
        .route("/api/policies/active", post(post_activate_policy))
        .route("/api/policies/active", delete(delete_deactivate_policy))
        .route("/api/policies/:version", get(get_policy))
        .route("/api/reasoner-connector-info", get(get_reasoner_connector_info))
        .route("/api/deliberation/task", post(post_exec_task))
        .route("/api/deliberation/data", post(post_access_data))
        .route("/api/deliberation/workflow", post(post_validate_workflow))
        .layer(TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Millis)))
        .with_state(state);

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
