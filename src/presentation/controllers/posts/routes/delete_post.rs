use std::str::FromStr;
use actix_web::{delete, web, HttpResponse};
use crate::services::use_cases::get_post::get_post;
use bson::oid::ObjectId;

#[delete("/delete/{id}")]
pub async fn delete_post_route(id: web::Path<String>) -> HttpResponse {
    let id = ObjectId::from_str(&id.into_inner()).unwrap();
    get_post(&id).await.unwrap();
    return HttpResponse::Ok()
        .content_type("text")
        .body("Completed");
}