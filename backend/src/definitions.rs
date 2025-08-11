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
pub struct TagID{
    uuid: String,
}

#[derive(Serialize,Deserialize)]
pub struct Tag{
    name: String,
    id: TagID,
}

pub struct TimeandUUID{
    pub time: DateTime<Utc>,
    pub uuid: Uuid,
}

pub struct GetTimeQuery{
    pub tx: oneshot::Sender<Result<TimeandUUID,std::io::Error>>,
}
