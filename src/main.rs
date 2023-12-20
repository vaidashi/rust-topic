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

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good.".to_string(),
        db: db_pool,
    });
    //Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                errors::AppErrorType::InvalidInput("Invalid JSON input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(tutor_routes)
            // .configure(topic_routes)
    };

    let host_port =
        env::var("SERVER_HOST_PORT").expect("SERVER_HOSTNAME_PORT is not set in .env file");

    //Start HTTP server
    HttpServer::new(app)
        .bind(host_port)
        .unwrap()
        .run()
        .await
}
