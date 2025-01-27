use crate::{
    models::prices::{Prices, QueryParam},
    services::prices::YahooFinanceService,
};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is a GET request response!")
}

#[get("/api/prices")]
pub async fn get_prices(
    query: Option<web::Query<QueryParam>>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = query
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing query parameter: ticker"))?;

    let ticker = query.ticker.trim();
    if ticker.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("Invalid empty ticker"));
    }

    let prices_data = YahooFinanceService::fetch_prices(ticker).await?;

    Ok(HttpResponse::Ok().json(Prices {
        ticker: ticker.to_string(),
        prices: prices_data,
    }))
}
