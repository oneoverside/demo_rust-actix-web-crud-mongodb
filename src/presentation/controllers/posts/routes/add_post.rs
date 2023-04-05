use actix_web::{post, web, HttpResponse};
use crate::entities::post::Post;
use crate::services::use_cases::add_post_use_case::add_post;
use crate::entities::errors::db_errors::Error;

#[post("/add")]
pub async fn add_post_route(data: web::Json<Post>) -> HttpResponse {
    return match add_post(&data.into_inner()).await{
        Ok(_) => HttpResponse::Ok()
            .content_type("text")
            .body("Successfully added"),
        Err(error) => match error {
            Error::ArgumentError(_) => HttpResponse::Ok()
                .content_type("text")
                .body("This element already exist"),
            Error::DbIsUnavailable(_) => HttpResponse::Ok()
                .content_type("text")
                .body("Db is unavailable"),
        }
    };
}