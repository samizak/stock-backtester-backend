mod prices;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(prices::hello).service(prices::get_prices);
}
