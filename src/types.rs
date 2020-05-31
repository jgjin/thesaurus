use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    #[serde(rename = "_id")]
    pub id: String,
    pub endpoint: Option<String>,
    pub synonyms: Vec<String>,
}

impl Word {
    pub fn new(word: &str) -> Self {
        Self {
            id: word.to_string(),
            endpoint: None,
            synonyms: Vec::new(),
        }
    }
}
