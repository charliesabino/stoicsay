use reqwest::Error;
use serde_derive::Deserialize;
use term_size;
use textwrap;
use tokio;

#[derive(Deserialize, Debug)]
struct Quote {
    text: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let quote = fetch_stoic_quote().await?;
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
    let wrapped_text = textwrap::fill(&quote.text, width);
    println!("{}\n- {}", wrapped_text, quote.author);
    Ok(())
}

async fn fetch_stoic_quote() -> Result<Quote, Error> {
    let url = "https://stoic-quotes.com/api/quote";
    let response = reqwest::get(url).await?.error_for_status()?;
    let quote: Quote = response.json().await?;
    Ok(quote)
}
