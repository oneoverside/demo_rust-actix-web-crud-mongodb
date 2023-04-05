use actix_web::{delete, web, HttpResponse};

#[delete("/delete/{id}")]
pub async fn delete_post(_: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text")
        .body("Post was succesfully deleted")
}