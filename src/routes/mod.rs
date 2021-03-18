mod emoji;
mod index;

use actix_web::web::ServiceConfig;

pub fn endpoints(config: &mut ServiceConfig) {
    index::endpoints(config);
    emoji::endpoints(config);
}
