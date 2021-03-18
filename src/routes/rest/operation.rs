use crate::model::{NewOperation, Operation};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_emoji)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/operations")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Operation::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/operations/{id}")]
pub async fn by_id(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Operation::by_id(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/operations/by-emoji/{id}")]
pub async fn by_emoji(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Operation::by_emoji(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/operations")]
pub async fn new(
    pool: web::Data<PgPool>,
    web::Json(item): web::Json<NewOperation>,
) -> Result<HttpResponse, Error> {
    let item = Operation::create(item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/operations/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
    web::Json(item): web::Json<NewOperation>,
) -> Result<HttpResponse, Error> {
    let item = Operation::update(id, item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/operations/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Operation::delete(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
