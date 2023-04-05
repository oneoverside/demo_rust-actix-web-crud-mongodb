use actix_web::web;

use crate::presentation::controllers::posts::routes::get_post::get_post_route;
use crate::presentation::controllers::posts::routes::add_post::add_post_route;
use crate::presentation::controllers::posts::routes::update_post::update_post;
use crate::presentation::controllers::posts::routes::delete_post::delete_post_route;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(get_post_route)
            .service(add_post_route)
            .service(delete_post_route)
            .service(update_post)
    );
}
