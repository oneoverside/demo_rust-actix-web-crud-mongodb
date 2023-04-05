use crate::entities::errors::db_errors::Error;
use crate::entities::post::Post;
use crate::repository::post_repository::PostRepository;

impl PostRepository {
    pub async fn insert(&self, post: &Post) -> Result<(), Error> {
        let c: bool = self.find_by_id(&post._id).await.unwrap().is_some();
        return match c {
            true => Err(Error::ArgumentError),
            false => match self.collection.insert_one(post, None).await {
                Ok(_) => Ok(()),
                Err(_) => return Err(Error::DbIsUnavailable),
            }
        }
    }
}