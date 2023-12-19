use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub level: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTopic {
    pub title: String,
    pub tutor_id: i32,
    pub description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTopic {
    pub title: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub duration: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<CreateTopic>> for CreateTopic {
    fn from(ct: web::Json<CreateTopic>) -> Self {
        CreateTopic {
            title: ct.title.clone(),
            tutor_id: ct.tutor_id,
            description: ct.description.clone(),
            format: ct.format.clone(),
            duration: ct.duration.clone(),
            level: ct.level.clone(),
        }
    }
}

impl From<web::Json<UpdateTopic>> for UpdateTopic {
    fn from(ut: web::Json<UpdateTopic>) -> Self {
        UpdateTopic {
            title: ut.title.clone(),
            description: ut.description.clone(),
            format: ut.format.clone(),
            duration: ut.duration.clone(),
            level: ut.level.clone(),
        }
    }
}