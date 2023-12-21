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

pub async fn post_new_tutor(
    new_tutor: web::Json<CreateTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppErrorType> {
    post_new_tutor_db(&app_state.db, new_tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, AppErrorType> {
    let tutor_id: i32 = params.0;

    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
    tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, AppErrorType> {
    let tutor_id: i32 = params.0;
    update_tutor_details_db(&app_state.db, tutor_id, tutor.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, AppErrorType> {
    let tutor_id: i32 = params.0;
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
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
    async fn get_all_tutors_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let resp = get_all_tutors(app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_tutor_details_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let params: web::Path<(i32,)> = web::Path::from((1,));

        let resp = get_tutor_details(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn post_tutor_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let new_tutor_payload = CreateTutor {
            first_name: "Phil".into(),
            last_name: "Collins".into(),
            email: "PCgenesismail.com".into(),
        };
        let params = web::Json(new_tutor_payload);

        let resp = post_new_tutor(params, app_state).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_tutor_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let update_tutor_payload = UpdateTutor {
            first_name: Some("Tom".into()),
            last_name: Some("Hanks".into()),
            email: Some("gump@bubbagump.com".into()),
        };
        let tutor_param = web::Json(update_tutor_payload);
        let params: web::Path<(i32,)> = web::Path::from((1,));

        let resp = update_tutor_details(app_state, params, tutor_param)
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[ignore]
    #[actix_rt::test]
    async fn delete_tutor_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let params: web::Path<(i32,)> = web::Path::from((1,));

        let resp = delete_tutor(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_tutor_detail_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });
        let params: web::Path<(i32,)> = web::Path::from((1000,));

        let resp = get_tutor_details(app_state, params).await;

        assert_eq!(resp.is_err(), true);
    }
    
    #[ignore]
    #[actix_rt::test]
    async fn delete_tutor_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: pool,
        });

        let params: web::Path<(i32,)> = web::Path::from((1000,));

        let resp = delete_tutor(app_state, params).await;

        assert_eq!(resp.is_err(), true);
    }
}
