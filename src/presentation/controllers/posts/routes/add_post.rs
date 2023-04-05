use actix_web::{post, web, HttpResponse};
use crate::entities::post::Post;
use crate::services::use_cases::add_post::add_post;
use crate::entities::errors::db_errors::Error;

#[post("/add")]
pub async fn add_post_route(data: web::Json<Post>) -> HttpResponse {
    return match add_post(&data.into_inner()).await{
        Ok(_) => HttpResponse::Ok()
            .content_type("text")
            .body("Successfully added"),
        Err(error) => match error {
            Error::ArgumentError => HttpResponse::Ok()
                .content_type("text")
                .body("This element isn't exist"),
            Error::DbIsUnavailable => HttpResponse::Ok()
                .content_type("text")
                .body("Db is unavailable")
        }
    };
}