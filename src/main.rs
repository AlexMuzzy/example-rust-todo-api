mod database;
mod handlers;
mod models;

use crate::database::create_connection_pool;
use crate::handlers::{
    todo_delete_handler, todo_get_handler, todo_post_handler, todo_put_handler, todos_get_handler,
};
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tower_http::LatencyUnit;
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

    // Create database connection pool
    let pool = create_connection_pool()
        .await
        .expect("Failed to create database pool");
    info!("Database connection pool created successfully");

    // Build the application with multiple routes
    let app = Router::new()
        .route("/", get(todos_get_handler))
        .route("/{id}", get(todo_get_handler))
        .route("/", post(todo_post_handler))
        .route("/{id}", put(todo_put_handler))
        .route("/{id}", delete(todo_delete_handler))
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(
                    trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind address on port 8080");
    axum::serve(listener, app)
        .await
        .expect("Failed to start application");
}
