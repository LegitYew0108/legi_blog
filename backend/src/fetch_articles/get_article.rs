use tracing::error;
use axum::{
    extract,
    http::StatusCode,
    response::Html,
};
use mongodb::bson::doc;
use crate::definitions::{ArticlePayload,RouterStatePayload};
use markdown::to_html;

pub async fn get_article(extract::State(payload): extract::State<RouterStatePayload>,extract::Path(article_id): extract::Path<String>)->Result<Html<String>,StatusCode>{
    let filter = doc!{"_id": &article_id};
    let Ok(article) = payload.db_client
        .database("test")
        .collection("articles")
        .find_one(filter).await else{
            error!("DB fetch failed: {}",article_id);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Some(article): Option<ArticlePayload> = article else{
        error!("Requested article was not found: {}",article_id);
        return Err(StatusCode::NOT_FOUND);
    };

    let html_article = to_html(&article.article);
    Ok(Html(html_article))
}
