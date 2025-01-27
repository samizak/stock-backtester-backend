use crate::{models::prices::PriceData, utils::cors::to_actix_error};
use actix_web::Error;
use yahoo_finance_api::YahooConnector;

pub struct YahooFinanceService;

impl YahooFinanceService {
    pub async fn fetch_prices(ticker: &str) -> Result<Vec<PriceData>, Error> {
        let provider = YahooConnector::new().map_err(to_actix_error)?;

        let response = provider
            .get_quote_range(ticker, "1d", "max")
            .await
            .map_err(to_actix_error)?;

        let quotes = response.quotes().map_err(to_actix_error)?;
        Self::process_quotes(&quotes)
    }

    fn process_quotes(quotes: &[yahoo_finance_api::Quote]) -> Result<Vec<PriceData>, Error> {
        use chrono::DateTime;

        quotes
            .iter()
            .map(|quote| {
                let datetime =
                    DateTime::from_timestamp(quote.timestamp as i64, 0).ok_or_else(|| {
                        actix_web::error::ErrorInternalServerError("Invalid timestamp")
                    })?;

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
}
