use bson::doc;
use bson::oid::ObjectId;
use crate::entities::post::Post;
use crate::repository::post_repository::PostRepository;

impl PostRepository {
    pub async fn get_by_id(&self, id: &ObjectId) -> Option<Post> {
        let filter = doc! { "_id": id };
        return self.collection.find_one(filter, None).await.unwrap();
    }
}