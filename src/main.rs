use reqwest::Error;
use serde_derive::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
struct Quote {
    text: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let quote = fetch_stoic_quote().await?;
    println!("\"{}\"\n- {}", quote.text, quote.author);

    Ok(())
}

async fn fetch_stoic_quote() -> Result<Quote, Error> {
    let url = "https://stoic-quotes.com/api/quote";
    let response = reqwest::get(url).await?.error_for_status()?;
    let quote: Quote = response.json().await?;
    Ok(quote)
}
