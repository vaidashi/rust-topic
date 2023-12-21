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

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: CreateTutor) -> Result<Tutor, AppErrorType> {
    let tutor_row = sqlx::query!(
        "INSERT INTO tutor (first_name, last_name, email) VALUES ($1, $2, $3) RETURNING id, first_name, last_name, email",
        new_tutor.first_name,
        new_tutor.last_name,
        new_tutor.email
    )
    .fetch_one(pool)
    .await?;

    Ok(Tutor {
        id: tutor_row.id,
        first_name: tutor_row.first_name.clone(),
        last_name: tutor_row.last_name.clone(),
        email: tutor_row.email.clone(),
    })
}

pub async fn get_tutor_details_db(
    pool: &PgPool,
    id: i32,
) -> Result<Tutor, AppErrorType> {
    let tutor_row = sqlx::query!(
        "SELECT id, first_name, last_name, email FROM tutor where id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
            id: tutor_row.id,
            first_name: tutor_row.first_name,
            last_name: tutor_row.last_name,
            email: tutor_row.email,
        }
    )
    .map_err(|_err| AppErrorType::NotFoundError("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, AppErrorType> {
    let tutor_row = sqlx::query!(
        "SELECT id, first_name, last_name, email FROM tutor where id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| AppErrorType::NotFoundError("Tutor id not found".into()))?;

    let new_tutor_record = Tutor {
        id: tutor_row.id,

        first_name: if let Some(first_name) = update_tutor.first_name {
            first_name
        } else {
            tutor_row.first_name
        },

        last_name: if let Some(last_name) = update_tutor.last_name {
            last_name
        } else {
            tutor_row.last_name
        },

        email: if let Some(email) = update_tutor.email{
            email
        } else {
            tutor_row.email
        },
    };

    let updated_tutor_row = sqlx::query!(
        "UPDATE tutor SET first_name = $1, last_name = $2, email = $3 where id = $4 returning id, first_name, last_name, email", 
        new_tutor_record.first_name, new_tutor_record.last_name, new_tutor_record.email, tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
            id: tutor_row.id,
            first_name: tutor_row.first_name,
            last_name: tutor_row.last_name,
            email: tutor_row.email,
        }
    )
    .map_err(|_err| AppErrorType::NotFoundError("Tutor id not found".into()))?;
    Ok(updated_tutor_row)
}

// pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, AppErrorType> {
//     let tutor_row = sqlx::query!(
//         "DELETE FROM tutor where id = $1",
//         tutor_id
//     )
//     .execute(pool)
//     .await
//     .map_err(|_err| AppErrorType::DbError("Unable to delete tutor ".into()))?;

//     Ok(format!("Deleted {} rows, record {:#?}", tutor_row.rows_affected(), tutor_row))
// }

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, AppErrorType> {
    // New query to delete related data from another table
    let related_topics_data_rows = sqlx::query!(
        "DELETE FROM topic where tutor_id = $1",
        tutor_id
    )
    .execute(pool)
    .await
    .map_err(|_err| AppErrorType::DbError("Unable to delete related data for tutor ".into()))?;

    let tutor_row = sqlx::query!(
        "DELETE FROM tutor where id = $1",
        tutor_id
    )
    .execute(pool)
    .await
    .map_err(|_err| AppErrorType::DbError("Unable to delete tutor ".into()))?;

    Ok(format!("Deleted {} rows from tutor and {} rows from topic, record {:#?}", tutor_row.rows_affected(), related_topics_data_rows.rows_affected(), tutor_row))
}