use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

pub fn articles_api() -> String {
  dotenv().ok(); // Load the environment variables from the .env file

  let url = "https://newsapi.org/v2/top-headlines?";
  let input_sources: Vec<&str> = vec!["science", "technology"];
  let api_key = std::env::var("NEWS_ORG_API_KEY").unwrap();
  let client = Client::new();

  for source in input_sources {
      let request_url = format!("{}category={}&apiKey={}", url, source, api_key);
      let response = client
          .get(&request_url)
          .header("User-Agent", "Rust POC")
          .send()
          .unwrap();

      let response_text = response.text().unwrap();
      println!("{}", response_text);
  }

  String::from("locochon")
}

fn main() {
  let locochon = articles_api();
}
