use crate::model::{Story, NewStory};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_title)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/stories")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Story::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/stories/{id}")]
pub async fn by_id(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Story::by_id(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/stories/by-title/{title}")]
pub async fn by_title(
    pool: web::Data<PgPool>,
    web::Path(title): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let item = Story::by_title(title, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/api/stories")]
pub async fn new(
    pool: web::Data<PgPool>,
    web::Json(item): web::Json<NewStory>,
) -> Result<HttpResponse, Error> {
    let item = Story::create(item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/foods/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
    web::Json(item): web::Json<NewStory>,
) -> Result<HttpResponse, Error> {
    let item = Story::update(id, item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/stories/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Story::delete(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
