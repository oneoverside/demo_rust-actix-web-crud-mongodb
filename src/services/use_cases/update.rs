use crate::entities::errors::db_errors::Error;
use crate::entities::post::Post;
use crate::repository::posts::post_repository::PostRepository;

pub async fn update(post: &Post) -> Result<(), Error> {
    return PostRepository::new().await.unwrap()
        .update(&post).await;
}
