use std::error::Error;
use colour::{dark_green, yellow, dark_red};
use newsapi::{articles_api, Articles};

fn render_articles(articles: &Articles) {
  for article in &articles.articles {
    dark_green!("> {}\n", article.title);
    yellow!("> {}\n", article.url);
    if let Some(author) = &article.author {
      dark_red!("> {}\n\n", author);
  } else {
      dark_red!("> No author available\n\n");
  }
  }

}

fn main() -> Result<(), Box<dyn Error>> {
  let results = articles_api(String::from("technology"))?;
  render_articles(&results);

  Ok(())
}
