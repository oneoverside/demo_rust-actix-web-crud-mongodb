use std::str::FromStr;
use actix_web::{delete, web, HttpResponse};
use bson::oid::ObjectId;
use crate::services::use_cases::delete_post::delete_post;

#[delete("/delete/{id}")]
pub async fn delete_post_route(id: web::Path<String>) -> HttpResponse {
    let id = ObjectId::from_str(&id.into_inner()).unwrap();
    delete_post(&id).await.unwrap();
    return HttpResponse::Ok()
        .content_type("text")
        .body("Completed");
}