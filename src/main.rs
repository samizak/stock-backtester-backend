use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use yahoo_finance_api::YahooConnector;

// Define the response structure
#[derive(Serialize)]
struct PriceData {
    datetime: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is a GET request response!")
}

#[get("/api/prices")]
async fn get_prices() -> Result<HttpResponse, actix_web::Error> {
    // Create a new Yahoo connector
    let provider = YahooConnector::new().unwrap();

    // Fetch historical data for AAPL asynchronously
    let response = provider
        .get_quote_range("AAPL", "1d", "max")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Get the price quotes
    let quotes = response
        .quotes()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    // Convert to our PriceData structure
    let prices: Vec<PriceData> = quotes
        .iter()
        .map(|quote| {
            let datetime = NaiveDateTime::from_timestamp(quote.timestamp as i64, 0) // Convert Unix timestamp
                .format("%Y-%m-%d %H:%M:%S") // Format as string
                .to_string();

            PriceData {
                datetime,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(prices))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(get_prices))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
