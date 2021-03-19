use actix_web::web::ServiceConfig;

mod crafting;
mod emoji;
mod food;
mod game;
mod helper;
mod operation;
mod player;
mod recipe;
mod story;

pub fn endpoints(config: &mut ServiceConfig) {
    emoji::endpoints(config);
    operation::endpoints(config);
    food::endpoints(config);
    recipe::endpoints(config);
    helper::endpoints(config);
}
