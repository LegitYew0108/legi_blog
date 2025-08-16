use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use tokio::sync::{mpsc, oneshot};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize)]
pub struct TagPayload{
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub name: String,
}

pub struct GetTimeQuery{
    pub tx: oneshot::Sender<Result<DateTime<Utc>,std::io::Error>>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PostArticle{
    pub title: String,
    pub article: String,
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
    pub timestamp: String,
    pub abstract_sentense: AbstractType,
    pub main_image: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct ArticlePayload{
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub metadata: ArticleMetadata,
    pub article: String,
}

#[derive(Debug,Clone)]
pub struct RouterStatePayload{
    pub time_tx: mpsc::Sender<GetTimeQuery>,
    pub db_client: mongodb::Client,
}

#[derive(Serialize,Deserialize,Debug)]
pub enum CardSortMethod{
    Latest(i64),
    Tag((ObjectId,i64)),
}

#[derive(Serialize,Deserialize)]
pub struct ArticleTag{
    pub article_id: ObjectId,
    pub tag_id: ObjectId,
}
