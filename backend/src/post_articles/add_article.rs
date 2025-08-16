use tracing::error;
use axum::{
    http::StatusCode,
    extract,
};
use crate::definitions::{PostArticle,AbstractType,ArticlePayload,ArticleMetadata,RouterStatePayload};

#[axum::debug_handler]
#[tracing::instrument(name="add_article")]
pub async fn add_article(extract::State(payload): extract::State<RouterStatePayload>, extract::Json(article_payload): extract::Json<PostArticle>)->StatusCode{
    let Ok(time) = crate::utils::time::get_time(payload.time_tx, 5).await else{
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    let metadata = ArticleMetadata{
        title: article_payload.title,
        timestamp: time.format("%Y-%m-%dT%H:%M:%s.%3fZ").to_string(),
        abstract_sentense: AbstractType::None,
        main_image: None,
    };

    let article_data = ArticlePayload{
        id: None,
        metadata,
        article: article_payload.article,
    };

    if let Err(e) = payload.db_client
        .database("test")
        .collection::<ArticlePayload>("articles")
        .insert_one(article_data).await {
        error!("Error while writing db:{}",e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    return StatusCode::OK;
}
