use crate::errors::AppErrorType;
use crate::models::tutor::{CreateTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, AppErrorType> {
    let tutor_rows =
        sqlx::query!("SELECT id, first_name, last_name, email FROM tutor")
            .fetch_all(pool)
            .await?;
 
    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor_row| Tutor {
            id: tutor_row.id,
            first_name: tutor_row.first_name.clone(),
            last_name: tutor_row.last_name.clone(),
            email: tutor_row.email.clone(),
        })
        .collect();

    match tutors.len() {
        0 => Err(AppErrorType::NotFoundError("No tutors found".into())),
        _ => Ok(tutors),
    }
}