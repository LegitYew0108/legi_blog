use serde::{Serialize,Deserialize};
use uuid::{NoContext, Timestamp, Uuid};
use chrono::prelude::*;
use tokio::sync::{mpsc, oneshot};

#[derive(Serialize,Deserialize)]
pub struct CardData{
    pub article_id: String,
    pub image_url: String,
    pub title: String,
    pub timestamp: String,
    pub abstract_sentense: String,
}

#[derive(Serialize,Deserialize)]
pub struct Tag{
    id: String,
    name: String,
}

pub struct TimeandUUID{
    pub time: DateTime<Utc>,
    pub uuid: Uuid,
}

pub struct GetTimeQuery{
    pub tx: oneshot::Sender<Result<TimeandUUID,std::io::Error>>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PostArticle{
    title: String,
    tags: Vec<String>,
    article: String,
}

#[derive(Serialize,Deserialize)]
pub enum AbstractType{
    None,
    Manual(String),
    Auto(Option<String>),
}

#[derive(Serialize,Deserialize)]
pub struct ArticleMetadata{
    pub title: String,
    pub tags: Vec<String>,
    pub timestamp: String,
    pub abstract_sentense: AbstractType,
    pub main_image: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct ArticlePayload{
    #[serde(rename = "_id", skip_serializing)]
    pub id: String,
    pub metadata: ArticleMetadata,
    pub article: String,
}
