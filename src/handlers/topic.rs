use crate::dbaccess::topic::*;
use crate::errors::AppErrorType;
use crate::models::topic::{CreateTopic, UpdateTopic};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

use super::tutor;

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

pub async fn post_new_topic (
    new_topic: web::Json<CreateTopic>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppErrorType> {
    post_new_topic_db(&app_state.db, new_topic.into_inner())
        .await
        .map(|topic| HttpResponse::Ok().json(topic))
}

pub async fn update_topic_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    topic: web::Json<UpdateTopic>,
) -> Result<HttpResponse, AppErrorType> {
    let (tutor_id, topic_id ) = (params.0, params.1);
    update_topic_details_db(&app_state.db, tutor_id, topic_id, topic.into_inner())
        .await
        .map(|topic| HttpResponse::Ok().json(topic))
}

pub async fn delete_topic(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppErrorType> {
    let (tutor_id, topic_id ) = (params.0, params.1);
    delete_topic_db(&app_state.db, tutor_id, topic_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_topics_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let resp = get_all_topics(app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_topic_details_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let params: web::Path<(i32,)> = web::Path::from((1, ));
        
        let resp = get_topic_details(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn post_topic_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let new_topic_payload = CreateTopic {
            tutor_id: 1,
            title: "Test topic".into(),
            topic_description: Some("This is a test topic".into()),
            format: None,
            topic_level: Some("Beginner".into()),
            duration: None,
        };
        let topic_param = web::Json(new_topic_payload);
        
        let resp = post_new_topic(topic_param, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_topic_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let update_topic_payload = UpdateTopic {
            title: Some("Updated title".into()),
            topic_description: Some("Updated topic description".into()),
            format: Some("Updated topic format".into()),
            duration: None,
            topic_level: Some("Updated topic level".into()),
        };
        let topic_param = web::Json(update_topic_payload);
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        
        let resp = update_topic_details(app_state, params, topic_param)
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_test_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        
        let resp = delete_topic(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_topic_details_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let params: web::Path<(i32,)> = web::Path::from((1000,));
        
        let resp = get_topic_details(app_state, params).await;

        assert_eq!(resp.is_err(), true);
    }

    #[actix_rt::test]
    async fn delete_test_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        
        let resp = delete_topic(app_state, params).await;

        assert_eq!(resp.is_err(), true);
    }
}