use mongodb::error::Error;
use mongodb::{
    bson::Document,
    Client
};


#[derive(Clone)]
pub struct Mongodb {
    client: Client,
}

impl Mongodb {
    pub async fn new() -> Mongodb {
        let new_client = Client::with_uri_str("mongodb://GENERAL_BOT:general%40bot@localhost:27017").await.expect("Erro ao conectar na base de dados");

        Mongodb {
            client: new_client
        }
    }

    pub async fn insert_one(&self, database: &str, collection: &str, data: Document) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);


        if let Err(e) = collection.insert_one(data, None).await {
            return Err(e);
        }

        Ok(())
    }
}
