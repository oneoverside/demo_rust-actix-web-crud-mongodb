use actix_web::{get, web, HttpResponse, Responder};
use bson::oid::ObjectId;

use crate::entities::post::Post;

#[get("/get/{id}")]
async fn get_post(_: web::Path<String>) -> impl Responder {
    let person = Post {
        _id: ObjectId::parse_str("507f1f77bcf86cd799439982").unwrap(),
        title: String::from("123"),
        connect: String::from(""),
        author: String::from(""),
        created_at: chrono::offset::Utc::now(),
        updated_at: chrono::offset::Utc::now(),
    };
    let json = serde_json::to_string(&person).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}