use crate::definitions::CardDatas;
use tracing::{debug, error};
use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
};

#[axum::debug_handler]
pub async fn serve_cards()->Result<Response,StatusCode>{
    let Ok(cards) = fetch_cards().await else{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    return Ok(cards.title.into_response());
}

/// fetch cards and 
async fn fetch_cards()->Result<CardDatas,std::io::Error>{
    // TODO: use DB
    let data = CardDatas{title:"card".to_string()};
    return Ok(data)
}
