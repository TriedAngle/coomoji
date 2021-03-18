use crate::model::{Food, NewFood};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_name)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/foods")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Food::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/foods/{id}")]
pub async fn by_id(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let item = Food::by_id(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/foods/by-name/{name}")]
pub async fn by_name(pool: web::Data<PgPool>, web::Path(name): web::Path<String>) -> Result<HttpResponse, Error> {
    let item = Food::by_name(name, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/foods")]
pub async fn new(pool: web::Data<PgPool>, web::Json(item): web::Json<NewFood>) -> Result<HttpResponse, Error> {
    let item = Food::create(item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/foods/{id}")]
pub async fn update(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>, web::Json(item): web::Json<NewFood>) -> Result<HttpResponse, Error> {
    let item = Food::update(id, item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/foods/{id}")]
pub async fn delete(pool: web::Data<PgPool>, web::Path(id): web::Path<i32>,) -> Result<HttpResponse, Error> {
    let item = Food::delete(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}