use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct PostArticle{
    title: String,
    tags: Vec<TagID>,
    article: String,
}
