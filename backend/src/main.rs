pub mod definitions;
mod fetch_articles;
mod post_articles;
mod utils;

use tracing::info;
use axum::{
    routing::{get,post},
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

    let (time_tx, mut time_rx) = mpsc::channel::<definitions::GetTimeQuery>(32);
    let time_get_task = tokio::spawn(utils::time::get_time_and_uuid_task(&mut time_rx));

    let app = Router::new()
        .route("/", get(||async {"Server active!"}))
        // データの取得系
        .route("/cards", get(fetch_articles::cards::serve_cards))
        .route("/articles/{article_id}", get())
        // データの投稿系
        .route("/add_article",post(post_articles::add_article::add_article))
        .with_state(time_tx.clone())
        .route("/add_tag",post(post_articles::add_tag::add_tag));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3440").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}
