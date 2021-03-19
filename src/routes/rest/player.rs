use crate::model::{NewPlayer, Player};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config.service(all).service(get).service(get_or_create);
}

#[get("/api/players")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Player::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/players/{id}")]
pub async fn get(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let player = Player::by_id(id.clone(), &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(player))
}

#[get("/api/players/gc/{id}")]
pub async fn get_or_create(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let maybe_player = Player::by_discord_id(id.clone(), &pool).await;
    match maybe_player {
        Ok(player) => Ok(HttpResponse::Ok().json(player)),
        Err(_) => {
            let new_player = NewPlayer { discord_id: id };
            let player = Player::create(new_player, &pool).await.unwrap();
            Ok(HttpResponse::Ok().json(player))
        }
    }
}
