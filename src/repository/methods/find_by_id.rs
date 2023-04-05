use bson::doc;
use bson::oid::ObjectId;
use crate::entities::post::Post;
use crate::repository::post_repository::PostRepository;

impl PostRepository {
    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<Post>, mongodb::error::Error> {
        let filter = doc! { "_id": id };
        let document = self.collection.find_one(filter, None).await?;

        match document {
            Some(post) => {
                Ok(Some(post))
            }
            None => Ok(None),
        }
    }
}