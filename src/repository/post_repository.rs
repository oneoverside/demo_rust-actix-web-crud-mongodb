use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use crate::entities::post::Post;

pub struct PostRepository {
    pub(crate) collection: Collection<Post>,
}

impl PostRepository {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let connection_string = "mongodb://localhost:27017";
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let collection = client.database("Test_Db").collection("posts");

        Ok(Self { collection })
    }
}