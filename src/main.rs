mod theme;

use newsapi::articles_api;
use newsapi::Articles;
use std::error::Error;

fn render_articles(articles: &Articles) {
  let theme = theme::default();
  theme.print_text("# Top headlines\n\n");
  for article in &articles.articles {
      theme.print_text(&format!("`{}`", article.title));
      theme.print_text(&format!("> *{}*", article.url));

      if let Some(author) = &article.author {
        theme.print_text(&format!("> Author: {:?}", author));
      } else {
        theme.print_text("> No author available");
      }
      theme.print_text("---");
  }
}

fn main() -> Result<(), Box<dyn Error>> {
    let results = articles_api(String::from("technology"))?;
    render_articles(&results);

    Ok(())
}
