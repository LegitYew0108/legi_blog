use crate::post_articles::definitions;
use axum::http::StatusCode;

#[tracing::instrument(name="add_article")]
pub async fn add_article()->StatusCode{

    return StatusCode::OK;
}
