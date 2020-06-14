#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use actix_web::{App, HttpServer};
use actix_redis::RedisSession;

mod db;
mod errors;
mod schema;
mod user;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    db::init();
    
    let app_host = std::env::var("APP_HOST").expect("APP_HOST not set");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not set");

    let redis_host = std::env::var("REDIS_HOST").expect("Redis host not set");
    let redis_port = std::env::var("REDIS_PORT").expect("Redis port not set");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32])
                    .cookie_name("session")
                    .cookie_max_age(chrono::Duration::days(1))
                    .cookie_secure(false), //set true for https only
                )
            .configure(user::route::init)
            .configure(auth::route::init)
    })
    .bind(format!("{}:{}", app_host, app_port))?;

    eprintln!("Server is running..");

    server.run().await
}