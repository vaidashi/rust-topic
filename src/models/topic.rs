use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Topic {
    pub id: i32,
    pub tutor_id: i32,
    pub title: String,
    pub topic_description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub topic_level: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTopic {
    pub title: String,
    pub tutor_id: i32,
    pub topic_description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub topic_level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTopic {
    pub title: Option<String>,
    pub topic_description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub topic_level: Option<String>,
}

impl From<web::Json<CreateTopic>> for CreateTopic {
    fn from(ct: web::Json<CreateTopic>) -> Self {
        CreateTopic {
            title: ct.title.clone(),
            tutor_id: ct.tutor_id,
            topic_description: ct.topic_description.clone(),
            format: ct.format.clone(),
            duration: ct.duration.clone(),
            topic_level: ct.topic_level.clone(),
        }
    }
}

impl From<web::Json<UpdateTopic>> for UpdateTopic {
    fn from(ut: web::Json<UpdateTopic>) -> Self {
        UpdateTopic {
            title: ut.title.clone(),
            topic_description: ut.topic_description.clone(),
            format: ut.format.clone(),
            duration: ut.duration.clone(),
            topic_level: ut.topic_level.clone(),
        }
    }
}
