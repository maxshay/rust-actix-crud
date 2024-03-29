#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod ops;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// https://yakshav.es/using-diesel-with-a-postgres-schema/
// https://app.planetscale.com/maksimshaynyuk/general-postgres/main

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = std::env::var("PORT").expect("$PORT is not set.");

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(handlers::index)
            .service(handlers::create)
            .service(handlers::show)
            .service(handlers::update)
            .service(handlers::destroy)
    })
    .bind(("localhost", port.parse().unwrap()))?
    .run()
    .await
}
