use actix_web::web::ServiceConfig;

mod emoji;
mod food;
mod helper;
mod operation;
mod recipe;

pub fn endpoints(config: &mut ServiceConfig) {
    emoji::endpoints(config);
    operation::endpoints(config);
    food::endpoints(config);
    recipe::endpoints(config);
    helper::endpoints(config);
}
