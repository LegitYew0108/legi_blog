mod cards;
pub mod definitions;

use tracing::info;
use axum::{
    routing::get,
    Router,
    response::{Response,IntoResponse},
    handler::Handler,
};

#[tokio::main]
async fn main() -> Result<(),std::io::Error>{
    tracing_subscriber::fmt()
        .init();
    info!("Server started");

    let app = Router::new()
        .route("/", get(||async {"Server active!"}))
        .route("/cards", get(cards::serve_cards));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3440").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}
