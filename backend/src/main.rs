/**fn main() {
    println!("Hello, world!");
} **/

use axum::{
routing::get, 
Json,
Router,
};
use serde:: Serialize;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn hello() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello from Rust API!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/hello", get(hello))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}