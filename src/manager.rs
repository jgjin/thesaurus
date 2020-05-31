use bson::to_bson;
use bson::{Bson, Document};
use mongodb::error;
use mongodb::results;

use crate::types::Word;

pub struct Manager {
    _mongo_client: mongodb::Client,
    frontier: mongodb::Collection,
}

impl Manager {
    pub async fn new() -> error::Result<Self> {
        let mongo_client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;

        let frontier = mongo_client.database("frontier").collection("words");
        frontier.delete_many(Document::new(), None).await?;

        Ok(Self {
            _mongo_client: mongo_client,
            frontier: frontier,
        })
    }

    // upsert and insert multiple method
    // insert word, upsert existing entry, insert synonyms, make sure errors are only duplicate keys
    pub async fn insert_many(
        &self,
        words: impl Iterator<Item = &String>,
    ) -> Result<Option<results::InsertManyResult>, String> {
        let documents: Vec<Document> = words
            .map(|word| {
                if let Bson::Document(document) = to_bson(&Word::new(word))
                    .map_err(|err| format!("error serializing BSON for {}: {}", word, err))?
                {
                    return Ok(document);
                }

                Err(format!(
                    "error converting BSON for {} to Mongo document",
                    word
                ))?
            })
            .collect::<Result<Vec<Document>, String>>()?;

        self.frontier
            .insert_many(documents, None)
            .await
            .map(|result| Some(result))
            .or_else(|err| match &*err.kind {
                error::ErrorKind::BulkWriteError(error::BulkWriteFailure {
                    write_errors: Some(errors),
                    write_concern_error: None,
                }) => match errors[..] {
                    [error::BulkWriteError {
                        index: _,
                        code: 11000,
                        code_name: _,
                        message: _,
                    }] => Ok(None),
                    _ => Err(err),
                },
                _ => Err(err),
            })
            .map_err(|err| format!("error inserting document: {}", err))
    }

    // pub async fn insert_one(&self, word: &str) -> Result<Option<results::InsertOneResult>, String> {
    //     let word_bson =
    //         to_bson(&Word::new(word)).map_err(|err| format!("error serializing BSON: {}", err))?;

    //     if let Bson::Document(document) = word_bson {
    //         return self
    //             .frontier
    //             .insert_one(document, None)
    //             .await
    //             .map(|result| Some(result))
    //             .or_else(|err| match *err.kind {
    //                 error::ErrorKind::WriteError(error::WriteFailure::WriteError(
    //                     error::WriteError {
    //                         code: 11000,
    //                         code_name: _,
    //                         message: _,
    //                     },
    //                 )) => Ok(None),
    //                 _ => Err(err),
    //             })
    //             .map_err(|err| format!("error inserting document: {}", err));
    //     }

    //     Err(format!("error converting BSON to Mongo document"))
    // }
}
