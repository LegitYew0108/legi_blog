use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct CardData{
    pub article_id: String,
    pub image_url: String,
    pub title: String,
    pub timestamp: String,
    pub abstract_sentense: String,
}
