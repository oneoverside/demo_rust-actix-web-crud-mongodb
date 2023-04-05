use bson::oid::ObjectId;
use crate::repository::post_repository::PostRepository;
use crate::entities::post::Post;

pub async fn get_post(id: &ObjectId) -> Option<Post> {
    return PostRepository::new().await.unwrap().get_by_id(&id).await;
}
