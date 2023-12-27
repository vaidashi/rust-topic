use crate::errors::AppErrorType;
use crate::models::topic::{CreateTopic, Topic, UpdateTopic};
use actix_web::App;
use chrono::{format, NaiveDateTime, Utc};
use sqlx::postgres::PgPool;

use super::tutor;

pub async fn get_all_topics_db(pool: &PgPool) -> Result<Vec<Topic>, AppErrorType> {
    let topic_rows = sqlx::query!("SELECT * FROM topic").fetch_all(pool).await?;

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

pub async fn get_topic_details_db(pool: &PgPool, topic_id: i32) -> Result<Topic, AppErrorType> {
    let topic_row = sqlx::query_as!(Topic, "SELECT * FROM topic where id = $1", topic_id)
        .fetch_optional(pool)
        .await?;

    if let Some(topic) = topic_row {
        Ok(topic)
    } else {
        Err(AppErrorType::NotFoundError(
            format!("No topic found for topic_id: {}", topic_id),
        ))
    }
}

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

pub async fn update_topic_details_db(
    pool: &PgPool,
    tutor_id: i32,
    topic_id: i32,
    update_topic: UpdateTopic,
) -> Result<Topic, AppErrorType> {
    let topic_row = sqlx::query_as!(
        Topic,
        "SELECT * FROM topic where id = $1 and tutor_id = $2",
        topic_id,
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| AppErrorType::NotFoundError("Topic id not found".into()))?;

    let update_time = Utc::now().naive_utc();

    let title: String = if let Some(title) = update_topic.title {
        title
    } else {
        topic_row.title
    };

    let topic_description: String = if let Some(topic_description) = update_topic.topic_description
    {
        topic_description
    } else {
        topic_row.topic_description.unwrap_or_default()
    };

    let format: String = if let Some(format) = update_topic.format {
        format
    } else {
        topic_row.format.unwrap_or_default()
    };

    let duration: String = if let Some(duration) = update_topic.duration {
        duration
    } else {
        topic_row.duration.unwrap_or_default()
    };

    let topic_level: String = if let Some(topic_level) = update_topic.topic_level {
        topic_level
    } else {
        topic_row.topic_level.unwrap_or_default()
    };

    let updated_topic_row = sqlx::query_as!(
        Topic,
        "UPDATE topic SET title = $1, topic_description = $2, format = $3, duration = $4, topic_level = $5, updated_at = $6 where id = $7 and tutor_id = $8 
        returning tutor_id, id, title, topic_description, duration, topic_level, format, created_at, updated_at", 
        title, topic_description, format, duration, topic_level, update_time, topic_id, tutor_id
    )
    .fetch_one(pool)
    .await;

    match updated_topic_row {
        Ok(topic_row) => Ok(topic_row),
        Err(_err) => Err(AppErrorType::NotFoundError("Topic id not found".into())),
    }
}

pub async fn delete_topic_db(
    pool: &PgPool,
    tutor_id: i32,
    topic_id: i32,
) -> Result<String, AppErrorType> {
    let topic_row = sqlx::query!(
        "DELETE FROM topic where id = $1 and tutor_id = $2 returning id",
        topic_id,
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| AppErrorType::NotFoundError("Topic id not found".into()))?;

    Ok(format!("Topic with id: {} deleted", topic_row.id))
}
