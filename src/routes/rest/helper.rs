use crate::model::{NewOperation, Operation};
use actix_web::web::ServiceConfig;
use actix_web::{delete, get, patch, post, web, Error, HttpRequest, HttpResponse, Responder};
use names::{Generator, Name};
use serde::Deserialize;
use sqlx::PgPool;

pub fn endpoints(config: &mut ServiceConfig) {
    config.service(random_name).service(random_name_numbered);
}

#[get("/api/helper/random/name")]
async fn random_name(
    pool: web::Data<PgPool>,
    web::Query(amount): web::Query<Amount>,
) -> Result<HttpResponse, Error> {
    let mut generator = Generator::default();
    if let Some(amount) = amount.amount {
        let mut random_names = vec![];
        for _ in 0..amount {
            random_names.push(generator.next().unwrap())
        }
        Ok(HttpResponse::Ok().json(random_names))
    } else {
        Ok(HttpResponse::Ok().json(generator.next().unwrap()))
    }
}

#[get("/api/helper/random/name-numbered")]
async fn random_name_numbered(
    pool: web::Data<PgPool>,
    web::Query(amount): web::Query<Amount>,
) -> Result<HttpResponse, Error> {
    let mut generator = Generator::with_naming(Name::Numbered);
    if let Some(amount) = amount.amount {
        let mut random_names = vec![];
        for _ in 0..amount {
            random_names.push(generator.next().unwrap())
        }
        Ok(HttpResponse::Ok().json(random_names))
    } else {
        Ok(HttpResponse::Ok().json(generator.next().unwrap()))
    }
}

#[derive(Deserialize)]
struct Amount {
    amount: Option<i32>,
}
