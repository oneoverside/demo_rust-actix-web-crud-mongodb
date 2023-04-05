use bson::doc;
use bson::oid::ObjectId;
use crate::repository::post_repository::PostRepository;

impl PostRepository {
    pub async fn delete_by_id(&self, id: &ObjectId) {
        let filter = doc! { "_id": id };
        self.collection.delete_one(filter, None).await.unwrap();
    }
}