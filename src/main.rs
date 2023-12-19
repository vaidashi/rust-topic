use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "./dbaccess/mod.rs"]
mod dbaccess;
#[path = "./errors.rs"]
mod errors;
#[path = "./handlers/mod.rs"]
mod handlers;
#[path = "./models/mod.rs"]
mod models;
#[path = "./routes.rs"]
mod routes;
#[path = "./state.rs"]
mod state;

use routes::*;
use state::AppState;

fn main() {
    println!("Hello, world!");
}
