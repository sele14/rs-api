#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

extern crate reqwest;
extern crate serde;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

// use serde::Deserialize;
// use dotenv::dotenv;
// use std::env;

// #[derive(Deserialize, Debug)]
struct Quote {
    // #[serde(rename = "01. symbol")]
    symbol: String,
    // #[serde(rename = "02. high")]
    high: f64,
    // #[serde(rename = "03. low")]
    low: f64,
    // #[serde(rename = "04. open")]
    open: f64,
    // #[serde(rename = "05. price")]
    price: f64,
    // #[serde(rename = "06. volume")]
    volume: i64,
    // #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    // #[serde(rename = "08. previous close")]
    previous_close: f64,
    // #[serde(rename = "09. change")]
    change: f64,
    // #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[tokio::main]
async fn fetch_data() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    let api_key = env::var("AV_API_KEY").expect("API Key not found in .enf file");
    let symbol = "AAPL";
    let request_url = format!("https://alphavantage.co/query?function=GLOBAL_QUOTE&symbol={symbol}&apikey={api_key}");
    println!("{}", request_url);
    let res = reqwest::get(&request_url).await?;
    // let quote: Quote = res.json().await?;
    let quote: Vec<Quote> = res.json().await?;

    println!("{:#?}", quote);

    Ok(())

}



fn main() {
    println!("Hello, world!");
    fetch_data();
}
