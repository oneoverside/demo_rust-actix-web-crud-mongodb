use bson::doc;
use crate::entities::errors::db_errors::Error;
use crate::entities::post::Post;
use crate::repository::posts::post_repository::PostRepository;

impl PostRepository {
    pub async fn update(&self, post: &Post) -> Result<(), Error> {
        let id_already_exist_in_db: bool = self.find_by_id(&post._id).await.unwrap().is_some();
        let query = doc! {"_id": &post._id};
        let update = doc! {"$set": {
            "connect": post.connect.clone(),
            "title": post.title.clone(),
            "author": post.author.clone(),
            "updated_at": format!("{:?}", post.updated_at),
            "created_at": format!("{:?}", post.updated_at),
        }};

        return match id_already_exist_in_db {
            false => Err(Error::ArgumentError),
            true => match self.collection.update_one(query, update, None).await {
                Ok(_) => Ok(()),
                Err(_) => return Err(Error::DbIsUnavailable),
            }
        }
    }
}