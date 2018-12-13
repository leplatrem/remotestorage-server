//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate r2d2;

use actix::prelude::SyncArbiter;
use actix_web::{http, middleware, server, App};
use api::{get_document, get_documents, update_document, AppState};
use db::DbExecutor;
use diesel::prelude::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

mod api;
mod db;
mod models;
mod schema;

fn main() {
    dotenv().ok();

    env_logger::init();

    let sys = actix::System::new("remotestorage-server");

    // Start 3 db executor actors
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let dbpool = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    let host = env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port);

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: dbpool.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            // API endpoints
            .resource("{folder:.*}/", |r| {
                r.method(http::Method::GET).with(get_documents)
            })
            .resource("{folder:.*}/{name}", |r| {
                r.method(http::Method::GET).with(get_document);
                r.method(http::Method::PUT).with(update_document);
            })
    })
    .bind(&addr)
    .unwrap()
    .start();

    println!("Started http server: {}", &addr);
    let _ = sys.run();
}
