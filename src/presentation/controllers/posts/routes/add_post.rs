use actix_web::{post, web, HttpResponse};
use crate::entities::post::Post;
use crate::services::use_cases::add_post_use_case::add_post_to_db_use_case;
use crate::entities::errors::db_errors::Error;

#[post("/add")]
pub async fn add_post(data: web::Json<Post>) -> HttpResponse {
    return match add_post_to_db_use_case(&data.into_inner()).await{
        Ok(_) =>
            HttpResponse::Ok()
                .content_type("text")
                .body("Succesfully added"),
        Err(error) => match error {
            Error::ArgumentError(_) =>
                HttpResponse::Ok()
                    .content_type("text")
                    .body("This element already exist"),
            Error::DbIsUnavailable(_) =>
                HttpResponse::Ok()
                    .content_type("text")
                    .body("Db is unavailable"),
        }
    };
}