mod cards;
pub mod definitions;
mod post_articles;
mod utils;

use tracing::info;
use axum::{
    routing::get,
    Router,
    response::{Response,IntoResponse},
    handler::Handler,
};
use tokio::sync::{mpsc,oneshot};
use chrono::prelude::*;
use uuid::{NoContext, Timestamp, Uuid};


#[tokio::main]
async fn main() -> Result<(),std::io::Error>{
    tracing_subscriber::fmt()
        .init();
    info!("Server started");

    let (tx, mut rx) = mpsc::channel::<definitions::GetTimeQuery>(32);
    let time_get_task = tokio::spawn(utils::time::get_time_and_uuid_task(&mut rx));

    let app = Router::new()
        .route("/", get(||async {"Server active!"}))
        // データの取得系
        .route("/cards", get(cards::serve_cards))
        // データの投稿系
        .route("/add_article",post(post_articles::add_article))
        .route("/add_tag",post(post_articles::add_tag));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3440").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}
