mod handlers;
mod models;

use crate::handlers::{
    init_todo_store, todo_delete_handler, todo_get_handler, todo_post_handler, todo_put_handler,
    todos_get_handler,
};
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    // Set up tracing subscriber for logging/observability
    fmt::Subscriber::builder()
        .with_max_level(LevelFilter::INFO)
        .init();

    info!("starting up");

    let mut store = init_todo_store();

    // Build the application with multiple routes
    let app = Router::new()
        .route("/", get(todos_get_handler))
        .route("/:id", get(todo_get_handler))
        .route("/", post(todo_post_handler))
        .route("/:id", put(todo_put_handler))
        .route("/:id", delete(todo_delete_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind address on port 8080");
    axum::serve(listener, app)
        .await
        .expect("Failed to start application");
}
