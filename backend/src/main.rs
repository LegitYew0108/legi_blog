pub mod definitions;
mod fetch_articles;
mod post_articles;
mod utils;

use tracing::info;
use axum::{
    routing::{get,post},
    Router,
};
use tokio::sync::mpsc;
use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(),std::io::Error>{
    tracing_subscriber::fmt()
        .init();
    info!("Server started");

    // .envファイルの読み込み
    dotenv().ok();

    // dbのクライアント設定
    let client_options = ClientOptions::parse(env::var("MONGODB_URI").expect("MONGODB_URI environment variable must be set.")).await.unwrap();
    let db_client = Client::with_options(client_options).unwrap();

    // 時間取得用タスク
    let (time_tx, mut time_rx) = mpsc::channel::<definitions::GetTimeQuery>(32);
    let _time_get_task = tokio::spawn(async move{utils::time::get_time_worker(&mut time_rx).await;});

    // dbと時間取得用タスクを各routeが得るためのstate
    let state_payload = definitions::RouterStatePayload{time_tx: time_tx.clone(),db_client};

    let app = Router::new()
        .route("/", get(||async {"Server active!"}))
        // データの取得系
        .route("/cards", get(fetch_articles::cards::serve_cards))
        .route("/articles/{article_id}", get(fetch_articles::get_article::get_article))
        // データの投稿系
        .route("/add_article",post(post_articles::add_article::add_article))
        .route("/add_tag",post(post_articles::add_tag::add_tag))
        .route("/attach_tag",post(post_articles::attach_tag::attach_tag))
        .with_state(state_payload);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3440").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
