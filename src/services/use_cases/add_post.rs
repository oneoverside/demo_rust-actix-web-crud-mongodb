use crate::repository::posts::post_repository::PostRepository;
use crate::entities::post::Post;
use crate::entities::errors::db_errors::Error;

pub async fn add_post(post: &Post) -> Result<(), Error> {
    return PostRepository::new().await.unwrap()
        .add(&post).await;
}