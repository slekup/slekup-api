use std::{path::PathBuf, sync::Arc};

use axum::{response::IntoResponse, routing::get, Json, Router};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use crate::state::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    Router::new()
        // Static files
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/api", get(root_handler))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
        )
}

pub async fn root_handler() -> impl IntoResponse {
    const MESSAGE: &str = "🚀 Welcome to Slekup API!";

    let json_response = serde_json::json!({
        "status": 200,
        "message": MESSAGE
    });

    Json(json_response)
}
