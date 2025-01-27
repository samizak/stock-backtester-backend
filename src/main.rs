use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
struct QueryParam {
    ticker: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is a GET request response!")
}

#[get("/api/prices")]
async fn get_prices(
    query: Option<web::Query<QueryParam>>,
) -> Result<HttpResponse, actix_web::Error> {
    // Handle Errors
    let query = match query {
        Some(q) => q,
        None => {
            return Err(actix_web::error::ErrorBadRequest(
                "Missing query parameter: ticker",
            ));
        }
    };

    if query.ticker.trim().is_empty() {
        return Err(actix_web::error::ErrorBadRequest(
            "Invalid ticker: ticker cannot be empty",
        ));
    }

    let ticker = &query.ticker;

    let provider = YahooConnector::new().unwrap();

    // Fetch historical data for AAPL asynchronously
    let response = provider
        .get_quote_range(ticker, "1d", "max")
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
            let datetime = DateTime::from_timestamp(quote.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m-%d")
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

fn calculate_rsi(closePrices: Vec<PriceData>, period: u32) {}
