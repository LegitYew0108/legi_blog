use crate::definitions::{ArticleMetadata,AbstractType};
use serde_json::json;
use tracing::{debug, info, error};
use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
};

#[axum::debug_handler]
#[tracing::instrument(name="serve_cards")]
pub async fn serve_cards()->Result<Response,StatusCode>{
    info!("cards fetch occured!");
    let Ok(cards) = fetch_cards().await else{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok(json_string) = serde_json::to_string(&cards) else{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    return Ok(json_string.into_response());
}

/// fetch cards and 
async fn fetch_cards()->Result<Vec<ArticleMetadata>,std::io::Error>{
    // TODO: use DB
    let data = vec![
        ArticleMetadata{title:"タイトル".to_string(),tags:vec!["aaa".to_string()], timestamp:"2000-03-15T05:20:10.123Z".to_string(), abstract_sentense:AbstractType::Manual("Response from server".to_string()),main_image:Some("https://placehold.jp/1920x1080.png".to_string())},
        ArticleMetadata{title:"タイトル".to_string(),tags:vec!["aaa".to_string()], timestamp:"2000-03-15T05:20:10.123Z".to_string(), abstract_sentense:AbstractType::Manual("Response from server".to_string()),main_image:Some("https://placehold.jp/1920x1080.png".to_string())},
        ArticleMetadata{title:"タイトル".to_string(),tags:vec!["aaa".to_string()], timestamp:"2000-03-15T05:20:10.123Z".to_string(), abstract_sentense:AbstractType::Manual("Response from server".to_string()),main_image:Some("https://placehold.jp/1920x1080.png".to_string())},
        ArticleMetadata{title:"タイトル".to_string(),tags:vec!["aaa".to_string()], timestamp:"2000-03-15T05:20:10.123Z".to_string(), abstract_sentense:AbstractType::Manual("Response from server".to_string()),main_image:Some("https://placehold.jp/1920x1080.png".to_string())},
    ];
    return Ok(data)
}
