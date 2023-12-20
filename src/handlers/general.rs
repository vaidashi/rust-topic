use crate::errors::AppErrorType;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let response_body = &app_state.health_check_response;
    HttpResponse::Ok().json(response_body)
}