mod cards;
pub mod definitions;
mod post_articles;

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

struct TimeandUUID{
    time: DateTime<Utc>,
    uuid: Uuid,
}

struct GetTimeQuery{
    tx: oneshot::Sender<Result<TimeandUUID,std::io::Error>>,
}

#[tokio::main]
async fn main() -> Result<(),std::io::Error>{
    tracing_subscriber::fmt()
        .init();
    info!("Server started");

    let app = Router::new()
        .route("/", get(||async {"Server active!"}))
        // データの取得系
        .route("/cards", get(cards::serve_cards))
        // データの投稿系
        .route("/add_article",post(post_articles::add_article));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3440").await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())
}

async fn get_time_and_uuid(rx: &mut mpsc::Receiver<GetTimeQuery>){
    loop{
        let query = rx.recv().await;
    }
}
