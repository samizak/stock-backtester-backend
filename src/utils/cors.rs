use actix_cors::Cors;

pub fn config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
        .max_age(3600)
}

pub fn to_actix_error<E: std::fmt::Display>(err: E) -> actix_web::Error {
    actix_web::error::ErrorInternalServerError(err.to_string())
}
