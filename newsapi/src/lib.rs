use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize};
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Articles {
  pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
  pub title: String,
  pub url: String,
  pub author: Option<String>,
}

pub fn articles_api(category: String) -> Result<Articles, Box<dyn Error>> {
  dotenv().ok();

  let url = "https://newsapi.org/v2/top-headlines?";
  let api_key = std::env::var("NEWS_ORG_API_KEY").unwrap();
  let client = Client::new();

  let request_url = format!("{}country=us&category={}&apiKey={}", url, category, api_key);
  let response = client
      .get(&request_url)
      .header("User-Agent", "Rust POC")
      .send()
      .unwrap();

  let source_response_text = response.text().unwrap();
  let articles: Articles = serde_json::from_str(&source_response_text)?;
  Ok(articles)
}
