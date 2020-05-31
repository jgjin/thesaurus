extern crate bson;
extern crate mongodb;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate tokio;

mod manager;
mod types;

use scraper::{Html, Selector};

use manager::Manager;

async fn parse_word(endpoint: &str) -> reqwest::Result<()> {
    let text = reqwest::get(&format!("https://www.thesaurus.com/browse/{}", endpoint,)[..])
        .await?
        .text()
        .await?;

    let html = Html::parse_document(&text[..]);

    let div_selector =
        Selector::parse("div.css-191l5o0-ClassicContentCard").expect("error in selector");
    let li_selector = Selector::parse("li").expect("error in selector");

    let div = html
        .select(&div_selector)
        .next()
        .expect("div.css-191l5o0-ClassicContentCard not found");

    let synonyms: Vec<String> = div
        .select(&li_selector)
        .map(|li| li.text().next().expect("no text found").to_string())
        .collect();

    println!("{:?}", synonyms);

    Ok(())
}

#[tokio::main]
async fn main() {
    let manager = Manager::new().await.expect("could not create Manager");

    println!(
        "{:?}",
        manager
            .insert_many(
                vec![
                    "hello".to_string(),
                    "hello".to_string(),
                    "hello".to_string(),
                    "world".to_string(),
                    "world".to_string(),
                    "world".to_string(),
                ]
                .iter()
            )
            .await
            .unwrap(),
    );
    println!(
        "{:?}",
        manager
            .insert_many(
                vec![
                    "hello".to_string(),
                    "hello".to_string(),
                    "hello".to_string(),
                    "hello".to_string(),
                    "world".to_string(),
                    "world".to_string(),
                    "world".to_string(),
                    "world".to_string(),
                ]
                .iter()
            )
            .await
            .unwrap(),
    );
}
