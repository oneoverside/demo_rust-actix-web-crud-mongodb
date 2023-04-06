use actix_web::{post, web, HttpResponse};
use crate::entities::errors::db_errors::Error;
use crate::entities::post::Post;
use crate::services::use_cases::update::update;

#[post("/update")]
pub async fn update_route(data: web::Json<Post>) -> HttpResponse {
    return match update(&data.into_inner()).await {
        Ok(_) => HttpResponse::Ok()
            .content_type("text")
            .body("Successfully added"),
        Err(error) => match error {
            Error::ArgumentError => HttpResponse::BadRequest()
                .content_type("text")
                .body("This element isn't exist"),
            Error::DbIsUnavailable => HttpResponse::InternalServerError()
                .content_type("text")
                .body("Db is unavailable")
        }
    };
}