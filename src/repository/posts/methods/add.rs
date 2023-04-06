use crate::entities::errors::db_errors::Error;
use crate::entities::post::Post;
use crate::repository::posts::post_repository::PostRepository;

impl PostRepository {
    pub async fn add(&self, post: &Post) -> Result<(), Error> {
        let id_already_exist_in_db: bool = self.find_by_id(&post._id).await.unwrap().is_some();
        return match id_already_exist_in_db {
            true => Err(Error::ArgumentError),
            false => match self.collection.insert_one(post, None).await {
                Ok(_) => Ok(()),
                Err(_) => return Err(Error::DbIsUnavailable),
            }
        }
    }
}
