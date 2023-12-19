use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tutor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTutor {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTutor>> for CreateTutor {
    fn from(ct: web::Json<CreateTutor>) -> Self {
        CreateTutor {
            first_name: ct.first_name.clone(),
            last_name: ct.last_name.clone(),
            email: ct.email.clone(),
            profile: ct.profile.clone(),
        }
    }
}   

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(ut: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            first_name: ut.first_name.clone(),
            last_name: ut.last_name.clone(),
            email: ut.email.clone(),
            profile: ut.profile.clone(),
        }
    }
}