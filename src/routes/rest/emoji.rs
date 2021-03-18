use crate::model::{Emoji, NewEmoji};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/emojis")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Emoji::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/emojis/{id}")]
pub async fn by_id(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Emoji::by_id(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/emojis")]
pub async fn new(
    pool: web::Data<PgPool>,
    web::Json(item): web::Json<NewEmoji>,
) -> Result<HttpResponse, Error> {
    let item = Emoji::create(item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/emojis/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
    web::Json(item): web::Json<NewEmoji>,
) -> Result<HttpResponse, Error> {
    let item = Emoji::update(id, item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/emojis/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Emoji::delete(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
