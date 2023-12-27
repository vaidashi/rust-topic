use crate::handlers::{general::*, topic::*, tutor::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn tutor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tutors")
            .route("/", web::get().to(get_all_tutors))
            .route("/", web::post().to(post_new_tutor))
            .route("/{tutor_id}", web::get().to(get_tutor_details))
            .route("/{tutor_id}", web::put().to(update_tutor_details))
            .route("/{tutor_id}", web::delete().to(delete_tutor))
            .route("/{tutor_id}/topics", web::get().to(get_topics_for_tutor))
            .route(
                "/{tutor_id}/{topic_id}",
                web::put().to(update_topic_details),
            )
            .route("/{tutor_id}/{topic_id}", web::delete().to(delete_topic)),
    );
}

pub fn topic_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/topics")
            .route("/", web::post().to(post_new_topic))
            .route("/", web::get().to(get_all_topics))
            .route("/{topic_id}", web::get().to(get_topic_details)),
    );
}
