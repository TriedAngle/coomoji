use crate::model::{Game, PlayerInventory, Recipe};
use actix_web::web::ServiceConfig;
use actix_web::{get, web, Error, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub fn endpoints(config: &mut ServiceConfig) {
    config.service(crafting);
}

#[get("/api/crafting/")]
async fn crafting(
    pool: web::Data<PgPool>,
    web::Json(crafting): web::Json<Crafting>,
) -> Result<HttpResponse, Error> {
    let player_inventory =
        PlayerInventory::by_game_and_player(crafting.game_id, crafting.player_id, &pool)
            .await
            .unwrap();

    let recipe =
        match Recipe::by_components_and_operation(crafting.operation, &crafting.items, &pool).await
        {
            Ok(recipe) => recipe,
            Err(err) => return Ok(actix_web::HttpResponse::NotFound().json("no matching recipe")),
        };

    let mut is_contained = true;
    for item in &crafting.items {
        if !player_inventory.inventory.contains(item) {
            is_contained = false;
            break;
        }
    }

    if !is_contained {
        return Ok(actix_web::HttpResponse::BadRequest()
            .json("items not in inventory, don't try to cheat ;)!"));
    }

    let mut new_inventory = player_inventory.inventory.clone();

    for item in &crafting.items {
        let index = new_inventory.iter().position(|x| *x == *item).unwrap();
        new_inventory.remove(index);
    }

    new_inventory.push(recipe.outcome);
    let game = PlayerInventory::update_inventory(player_inventory.id, &new_inventory, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(game))
}

#[derive(Deserialize, Clone)]
pub struct Crafting {
    pub game_id: Uuid,
    pub player_id: i32,
    pub operation: i32,
    pub items: Vec<i32>,
}
