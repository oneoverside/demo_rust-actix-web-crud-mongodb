use bson::oid::ObjectId;
use crate::repository::post_repository::PostRepository;
use crate::entities::errors::db_errors::Error;

pub async fn delete_post(id: &ObjectId) -> Result<(), Error> {
    PostRepository::new().await.unwrap().delete_by_id(&id).await;
    return Ok(());
}
