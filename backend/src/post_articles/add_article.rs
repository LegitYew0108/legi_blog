use axum::{
    http::StatusCode,
    extract,
};
use crate::definitions::{PostArticle,GetTimeQuery};
use tokio::sync::mpsc;

#[tracing::instrument(name="add_article")]
#[axum::debug_handler]
pub async fn add_article(extract::State(time_tx): extract::State<mpsc::Sender<GetTimeQuery>>, extract::Json(article_payload): extract::Json<PostArticle>)->StatusCode{

    return StatusCode::OK;
}
