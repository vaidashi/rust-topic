use crate::dbaccess::tutor::*;
use crate::errors::AppErrorType;
use crate::models::tutor::{CreateTutor, Tutor, UpdateTutor};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, AppErrorType> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}