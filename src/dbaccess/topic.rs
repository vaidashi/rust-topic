use crate::errors::AppErrorType;
use crate::models::topic::{Topic, CreateTopic, UpdateTopic};
use sqlx::postgres::PgPool;
use chrono::{Utc, NaiveDateTime};

pub async fn get_all_topics_db(pool: &PgPool) -> Result<Vec<Topic>, AppErrorType> {
    let topic_rows =
        sqlx::query!("SELECT * FROM topic")
            .fetch_all(pool)
            .await?;

    let topics: Vec<Topic> = topic_rows
        .iter()
        .map(|topic_row| Topic {
            id: topic_row.id,
            tutor_id: topic_row.tutor_id,
            title: topic_row.title.clone(),
            topic_description: topic_row.topic_description.clone(),
            format: topic_row.format.clone(),
            duration: topic_row.duration.clone(),
            topic_level: topic_row.topic_level.clone(),
            created_at: topic_row.created_at,
            updated_at: topic_row.updated_at,
        })
        .collect();

    match topics.len() {
        0 => Err(AppErrorType::NotFoundError("No topics found".into())),
        _ => Ok(topics),
    }
}

pub async fn get_topics_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Topic>, AppErrorType> {
    let topic_rows = sqlx::query_as!(
        Topic,
        "SELECT * FROM topic where tutor_id = $1 order by id desc",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(topic_rows)
}

pub async fn get_topic_details_db(
    pool: &PgPool,
    topic_id: i32,
) -> Result<Topic, AppErrorType> {
    let topic_row = sqlx::query_as!(
        Topic,
        "SELECT * FROM topic where id = $1",
        topic_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(topic) = topic_row {
        Ok(topic)
    } else {
        Err(AppErrorType::NotFoundError(
            // "No topic found".to_string(),
            format!("No topic found for topic_id: {}", topic_id)
        ))
    }
}

// pub async fn post_new_topic_db(
//     pool: &PgPool,
//     new_topic: CreateTopic,
// ) -> Result<Topic, AppErrorType> {
//     let topic_row = sqlx::query_as!(
//         Topic,
//         "INSERT INTO topic (
//             tutor_id, title, topic_description, format, duration, topic_level)
//             values ($1,$2,$3,$4,$5,$6) 
//             returning tutor_id, id, title, topic_description, duration, topic_level, format", 
//     new_topic.tutor_id, new_topic.title, new_topic.topic_description, new_topic.format, new_topic.duration, new_topic.topic_level)
//     .fetch_one(pool)
//     .await?;

//     Ok(topic_row)
// }

pub async fn post_new_topic_db(
    pool: &PgPool,
    new_topic: CreateTopic,
) -> Result<Topic, AppErrorType> {
    let current_time = Utc::now().naive_utc(); // Get the current time

    let topic_row = sqlx::query_as!(
        Topic,
        "INSERT INTO topic (
            tutor_id, title, topic_description, format, duration, topic_level, created_at, updated_at)
            values ($1,$2,$3,$4,$5,$6,$7,$8) 
            returning tutor_id, id, title, topic_description, duration, topic_level, format, created_at, updated_at", 
    new_topic.tutor_id, new_topic.title, new_topic.topic_description, new_topic.format, new_topic.duration, new_topic.topic_level, current_time, current_time)
    .fetch_one(pool)
    .await?;

    Ok(topic_row)
}