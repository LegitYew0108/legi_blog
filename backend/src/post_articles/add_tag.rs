use tracing::error;
use axum::{
    http::StatusCode,
    extract,
};
use crate::definitions::{TagPayload,RouterStatePayload};

#[axum::debug_handler]
#[tracing::instrument(name="add_article")]
pub async fn add_tag(extract::State(payload):extract::State<RouterStatePayload>,extract::Json(tag_name):extract::Json<String>)->StatusCode{
    let tag = TagPayload{id:None,name:tag_name};

    if let Err(e) = payload.db_client
        .database("test")
        .collection::<TagPayload>("tags")
        .insert_one(tag).await {
        error!("Error while writing db:{}",e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    return StatusCode::OK;
}
