pub mod config; // This line is added to the main.rs file
pub mod items; // This line is added to the main.rs file

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // get logs
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    println!("Starting server on {}", 3000);
    let app = Router::new()
        // middleware to allow CORS
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ]))
        // add the items routes
        .route("/", get(|| async { "Hello World" }))
        .route("/item", post(items::handler::insert_one_item))
        .route("/item", get(items::handler::find_items))
        .route("/item/:id", get(items::handler::find_one_item));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
