use std::str::FromStr;
use actix_web::{get, web, HttpResponse, Responder};
use bson::oid::ObjectId;
use crate::services::use_cases::get_post::get_post;

#[get("/get/{id}")]
pub(crate) async fn get_post_route(id: web::Path<String>) -> impl Responder {
    let id = ObjectId::from_str(&id.into_inner()).unwrap();
    return match get_post(&id).await {
        Some(value) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&value).unwrap())
        },
        None => HttpResponse::Ok().body("Post with this id isn't exist"),
    };
}