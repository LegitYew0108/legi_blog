use tracing::error;
use mongodb::bson::RawDocumentBuf;
use axum::{
    extract,
    http::StatusCode,
};
use crate::definitions::RouterStatePayload;

pub async fn attach_tag(extract::State(payload):extract::State<RouterStatePayload>, extract::Json(new_relation):extract::Json<RawDocumentBuf>)->StatusCode{
    if let Err(e) = payload.db_client
        .database("test")
        .collection::<RawDocumentBuf>("articletag")
        .insert_one(new_relation).await {
        error!("Error while writing db:{}",e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    return StatusCode::OK;
}
