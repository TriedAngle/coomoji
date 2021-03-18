use actix_web::web::ServiceConfig;

mod index;
mod rest;

pub fn endpoints(config: &mut ServiceConfig) {
    index::endpoints(config);
    rest::endpoints(config);
}
