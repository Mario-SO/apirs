// paper.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct PublishPaperRequest {
    #[validate(length(min = 3, message = "Author name is too short"))]
    pub paper_author: String,
    #[validate(length(min = 5, message = "Paper title is too short"))]
    pub paper_title: String,
    #[validate(length(min = 5, message = "Paper topic is too short"))]
    pub paper_topic: String,
}
#[derive(Validate, Deserialize, Serialize)]
pub struct UpdatePaperURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct Paper {
    pub paper_author: String,
    pub paper_title: String,
    pub paper_topic: String,
    pub paper_upvotes: String,
    pub paper_downvotes: String,
}

impl Paper {
    pub fn new(
        paper_author: String,
        paper_title: String,
        paper_topic: String,
        paper_upvotes: String,
        paper_downvotes: String,
    ) -> Paper {
        Paper {
            paper_author,
            paper_title,
            paper_topic,
            paper_upvotes,
            paper_downvotes,
        }
    }
}
