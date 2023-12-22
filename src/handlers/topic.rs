use crate::dbaccess::topic::*;
use crate::errors::AppErrorType;
use crate::models::topic::{CreateTopic, UpdateTopic};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_topics(app_state: web::Data<AppState>) -> Result<HttpResponse, AppErrorType> {
    get_all_topics_db(&app_state.db)
        .await
        .map(|topics| HttpResponse::Ok().json(topics))
}

pub async fn get_topics_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, AppErrorType> {
    let tuple = params.0;
    let tutor_id: i32 = tuple;

    get_topics_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_topic_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, AppErrorType> {
    let (topic_id) = params.0;
    get_topic_details_db(&app_state.db,  topic_id)
        .await
        .map(|topic| HttpResponse::Ok().json(topic))
}

/* 
curl -X POST localhost:3000/topics/ -H "Content-Type: application/json" -d '{"tutor_id":1, "title":"test title", "topic_description":"test description", "format":"test format", "duration": 60, "topic_level":"test level"}'
*/

pub async fn post_new_topic (
    new_topic: web::Json<CreateTopic>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppErrorType> {
    post_new_topic_db(&app_state.db, new_topic.into_inner())
        .await
        .map(|topic| HttpResponse::Ok().json(topic))
}