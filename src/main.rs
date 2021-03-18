#![allow(unused)]

mod config;
mod db;
mod model;
mod routes;

use crate::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use listenfd::ListenFd;
use sqlx::{PgPool, Postgres};

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let mut listenfd = ListenFd::from_env();

    let pool = PgPool::new(&config.db_address).await?;
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive().max_age(3600))
            .data(pool.clone())
            .configure(routes::endpoints)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(config.address)?,
    };

    server.run().await?;

    Ok(())
}
