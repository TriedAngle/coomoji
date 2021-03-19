use crate::model::{NewRecipe, Recipe};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config
        .service(all)
        .service(by_id)
        .service(by_operation)
        .service(by_outcome)
        .service(by_components)
        .service(by_operation_components)
        .service(new)
        .service(update)
        .service(delete);
}

#[get("/api/recipes")]
pub async fn all(pool: web::Data<PgPool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.query_string().is_empty() {
        let items = Recipe::all(&pool).await.unwrap();
        Ok(HttpResponse::Ok().json(items))
    } else {
        unimplemented!()
    }
}

#[get("/api/recipes/{id}")]
pub async fn by_id(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Recipe::by_id(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[get("/api/recipes/by-operation/{id}")]
pub async fn by_operation(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let items = Recipe::by_operation(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/recipes/by-outcome/{id}")]
pub async fn by_outcome(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let items = Recipe::by_outcome(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/recipes/by-outcome-components/{id}")]
pub async fn by_operation_components(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
    web::Json(components): web::Json<Vec<i32>>,
) -> Result<HttpResponse, Error> {
    let items = Recipe::by_components_and_operation(id, &components, &pool)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(items))
}

#[get("/api/recipes/by-components")]
pub async fn by_components(
    pool: web::Data<PgPool>,
    web::Json(components): web::Json<Vec<i32>>,
) -> Result<HttpResponse, Error> {
    let items = Recipe::by_components(&components, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(items))
}

#[post("/api/recipes")]
pub async fn new(
    pool: web::Data<PgPool>,
    web::Json(item): web::Json<NewRecipe>,
) -> Result<HttpResponse, Error> {
    let item = Recipe::create(item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[patch("/api/recipes/{id}")]
pub async fn update(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
    web::Json(item): web::Json<NewRecipe>,
) -> Result<HttpResponse, Error> {
    let item = Recipe::update(id, item, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
#[delete("/api/recipes/{id}")]
pub async fn delete(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let item = Recipe::delete(id, &pool).await.unwrap();
    Ok(HttpResponse::Ok().json(item))
}
