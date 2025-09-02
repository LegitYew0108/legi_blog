use tracing::error;
use axum::{
    extract,
    http::StatusCode,
    response::Html,
};
use mongodb::bson::{doc, oid::ObjectId};
use crate::definitions::{ArticlePayload,RouterStatePayload};
use markdown::to_html;
use std::str::FromStr;

pub async fn get_article(extract::State(payload): extract::State<RouterStatePayload>,extract::Path(article_id): extract::Path<String>)->Result<Html<String>,StatusCode>{
    let Ok(oid) = ObjectId::from_str(&article_id) else{
        error!("this path is not objectID!");
        return Err(StatusCode::NOT_FOUND);
    };
    let filter = doc!{"_id": oid};
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
