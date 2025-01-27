use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use yahoo_finance_api::YahooConnector;

// use anyhow::Result;
// use sqlx::{postgres::PgPoolOptions, Row};

#[derive(Serialize)]
struct Prices {
    ticker: String,
    prices: Vec<PriceData>,
}
#[derive(Serialize)]
struct PriceData {
    date: String,
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
    let query = query
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing query parameter: ticker"))?;

    let ticker = query.ticker.trim();
    if ticker.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("Invalid empty ticker"));
    }

    let prices = fetch_prices(ticker).await?;
    Ok(HttpResponse::Ok().json(prices))
}

async fn fetch_prices(ticker: &str) -> Result<Prices, actix_web::Error> {
    let provider = YahooConnector::new().map_err(to_actix_error)?;

    let response = provider
        .get_quote_range(ticker, "1d", "max")
        .await
        .map_err(to_actix_error)?;

    let quotes = response.quotes().map_err(to_actix_error)?;
    let prices_data = process_quotes(&quotes)?;

    Ok(Prices {
        ticker: ticker.to_string(),
        prices: prices_data,
    })
}

fn process_quotes(quotes: &[yahoo_finance_api::Quote]) -> Result<Vec<PriceData>, actix_web::Error> {
    quotes
        .iter()
        .map(|quote| {
            let datetime = DateTime::from_timestamp(quote.timestamp as i64, 0)
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("Invalid timestamp"))?;

            Ok(PriceData {
                date: datetime.format("%Y-%m-%d").to_string(),
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            })
        })
        .collect()
}

fn to_actix_error<E: std::fmt::Display>(err: E) -> actix_web::Error {
    actix_web::error::ErrorInternalServerError(err.to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(cors_config())
            .service(hello)
            .service(get_prices)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
        .max_age(3600)
}
