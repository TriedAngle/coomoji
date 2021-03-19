use crate::model::{Game, NewGame, NewPlayer, Player, PlayerInventory, Story};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use std::future::Future;
use uuid::Uuid;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_channel)
        .service(new)
        .service(take)
        .service(give)
        .service(join);
}

#[get("/api/games")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Game::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/games/by-channel/{}")]
pub async fn by_channel(
    pool: web::Data<PgPool>,
    web::Json(new_game): web::Json<NewGame>,
) -> Result<HttpResponse, Error> {
    let game = Game::by_channel_id(new_game.channel_id.clone(), &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(game))
}

#[post("/api/games")]
pub async fn new(
    pool: web::Data<PgPool>,
    web::Json(new_game): web::Json<NewGame>,
) -> Result<HttpResponse, Error> {
    match Game::by_channel_id(new_game.channel_id.clone(), &pool).await {
        Ok(game) => return Ok(HttpResponse::BadRequest().json("this channel already has a game")),
        Err(_) => {
            let game = Game::create(new_game, &pool).await.unwrap();
            Ok(HttpResponse::Ok().json(game))
        }
    }
}

#[post("/api/games/{id}/take/{player_id}/{item_id}")]
pub async fn take(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    web::Path(player_id): web::Path<i32>,
    web::Path(item_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut game = Game::by_id(id.clone(), &pool).await.unwrap();
    let mut player_inventory = PlayerInventory::by_game_and_player(game.id, player_id, &pool)
        .await
        .unwrap();
    match game.inventory.iter().position(|x| *x == item_id) {
        Some(id) => game.inventory.remove(id),
        None => return Ok(HttpResponse::BadRequest().json("Item not in inventory")),
    };
    player_inventory.inventory.push(item_id);
    Game::update_inventory(game.id, &game.inventory, &pool)
        .await
        .unwrap();
    PlayerInventory::update_inventory(player_inventory.id, &player_inventory.inventory, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json("Success"))
}

#[post("/api/games/{id}/give/{player_id}/{item_id}")]
pub async fn give(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    web::Path(player_id): web::Path<i32>,
    web::Path(item_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut game = Game::by_id(id.clone(), &pool).await.unwrap();
    let mut player_inventory = PlayerInventory::by_game_and_player(game.id, player_id, &pool)
        .await
        .unwrap();
    match player_inventory
        .inventory
        .iter()
        .position(|x| *x == item_id)
    {
        Some(id) => game.inventory.remove(id),
        None => return Ok(HttpResponse::BadRequest().json("Item not in inventory")),
    };
    game.inventory.push(item_id);
    Game::update_inventory(game.id, &game.inventory, &pool)
        .await
        .unwrap();
    PlayerInventory::update_inventory(player_inventory.id, &player_inventory.inventory, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json("Success"))
}

#[post("/api/games/{id}/join/{player}")]
pub async fn join(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    web::Path(player_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut game = Game::by_id(id.clone(), &pool).await.unwrap();
    if PlayerInventory::by_game_and_player(game.id.clone(), player_id, &pool)
        .await
        .is_ok()
    {
        return Ok(HttpResponse::BadRequest().json("Player already in game"));
    }

    let player_inventory = PlayerInventory::create(game.id.clone(), player_id, &pool)
        .await
        .unwrap();
    game.player_inventories.push(player_inventory.id);
    game.players.push(player_id);
    let game = Game::update_players(game.id, &game.players, &game.player_inventories, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(game))
}

#[post("/api/games/{id}/leave/{player}")]
pub async fn leave(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<Uuid>,
    web::Path(player_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let mut game = Game::by_id(id.clone(), &pool).await.unwrap();
    if PlayerInventory::by_game_and_player(game.id.clone(), player_id, &pool)
        .await
        .is_err()
    {
        return Ok(HttpResponse::BadRequest().json("Player not in game"));
    }

    let player_inventory = PlayerInventory::by_game_and_player(game.id.clone(), player_id, &pool)
        .await
        .unwrap();
    PlayerInventory::delete(player_inventory.id, &pool)
        .await
        .unwrap();

    let index = game.players.iter().position(|x| *x == player_id).unwrap();
    game.players.remove(index);

    let index = game
        .player_inventories
        .iter()
        .position(|x| *x == player_inventory.id)
        .unwrap();
    game.player_inventories.remove(index);

    let game = Game::update_players(game.id, &game.players, &game.player_inventories, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(game))
}
