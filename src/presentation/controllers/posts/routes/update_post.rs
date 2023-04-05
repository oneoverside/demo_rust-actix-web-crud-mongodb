use actix_web::{post, web, HttpResponse};
use crate::entities::post::Post;

#[post("/update")]
pub async fn update_post(data: web::Json<Post>) -> HttpResponse {
    let person: Post = data.into_inner();
    let json = serde_json::to_string(&person).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}