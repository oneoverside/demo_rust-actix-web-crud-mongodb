use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use bson::doc;
use bson::oid::ObjectId;
use crate::entities::post::Post;
use crate::entities::errors::db_errors::Error;

pub struct PostRepository {
    collection: Collection<Post>,
}

impl PostRepository {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let connection_string = "mongodb://localhost:27017";
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let collection = client.database("Test_Db").collection("posts");

        Ok(Self { collection })
    }

    pub async fn insert(&self, post: &Post) -> Result<(), Error> {
        let c: bool = self.find_by_id(&post._id).await.unwrap().is_some();
        return match c {
            true => Err(Error::ArgumentError(String::from("Post with this Id already exist"))),
            false => match self.collection.insert_one(post, None).await {
                Ok(_) => Ok(()),
                Err(_) => return Err(Error::DbIsUnavailable(String::from(""))),
            }
        }
    }

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