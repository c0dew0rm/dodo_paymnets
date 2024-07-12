#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod schema;
mod models;
mod handlers;
mod utils;

use handlers::user::{register_user, login_user, get_user_profile};
use handlers::transaction::{create_transaction, get_transactions};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let state = handlers::user::AppState {
        pool: web::Data::new(pool.clone()),
        secret_key: secret_key.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default())
            .app_data(web::Data::new(state.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/profile", web::post().to(get_user_profile))
            .route("/transactions", web::post().to(create_transaction))
            .route("/transactions", web::get().to(get_transactions))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
