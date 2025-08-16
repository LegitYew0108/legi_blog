use crate::definitions::{ArticleMetadata,RouterStatePayload,CardSortMethod};
use tracing::{warn, info, error};
use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
    extract
};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;

#[axum::debug_handler]
#[tracing::instrument(name="serve_cards")]
pub async fn serve_cards(extract::State(payload): extract::State<RouterStatePayload>, extract::Query(sort_method):extract::Query<CardSortMethod>)->Result<Response,StatusCode>{
    info!("cards fetch occured!");
    let Ok(cards) = fetch_cards(payload.db_client, sort_method).await else{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok(json_string) = serde_json::to_string(&cards) else{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    return Ok(json_string.into_response());
}

/// fetch cards and 
async fn fetch_cards(db_client: mongodb::Client,sort_method: CardSortMethod)->Result<Vec<ArticleMetadata>,std::io::Error>{
    let limit_num;
    let filter = match sort_method{
        CardSortMethod::Latest{card_num}=>{
            limit_num = card_num;
            doc!{}
        },
        CardSortMethod::Tag{tag_id,card_num}=>{
            limit_num = card_num;
            doc!{"metadata": doc!{"tags": doc!{"$all": tag_id}}}
        }
    };
    let Ok(cursor):Result<mongodb::Cursor<ArticleMetadata>, mongodb::error::Error> = db_client.database("test")
        .collection("articles")
        .find(filter)
        .sort(doc!{"id_": 1})
        .limit(limit_num.into())
        .projection(doc!{"metadata":"true"}).await else{
        error!("failed to fetch cards from db");
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"failed to fetch cards from db"));
    };
    let Ok(cards) = cursor.try_collect().await else{
        warn!("");
        return Err(std::io::Error::new(std::io::ErrorKind::Other,"failed to fetch cards from db"));
    };
    return Ok(cards)
}
