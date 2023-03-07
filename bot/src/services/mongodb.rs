use futures::{StreamExt};
use mongodb::{
    bson::Document,
    error:: Error,
    Client,
    Collection
};


#[derive(Clone)]
pub struct Mongodb {
    client: Client,
}

impl Mongodb {
    pub async fn new() -> Mongodb {
        let new_client = Client::with_uri_str("mongodb://GENERAL_BOT:general%40bot@general-bot-database:27017").await.expect("Erro ao conectar na base de dados");

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

    pub async fn get_one(&self, database: &str, collection: &str, filter: Document) -> Option<Document> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        collection.find_one(filter, None).await.ok()?
    }

    pub async fn get_collection<T>(&self, database: &str, collection: &str) -> Collection<T> {
        let db = self.client.database(database);

        db.collection::<T>(collection)
    }

    pub async fn get(&self, database: &str, collection: &str, filter: Document) -> Option<Vec<Document>>{
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);


        let cursor = collection.find(filter, None).await;

        match cursor {
            Ok(mut cur) => {
                let mut vec_doc: Vec<Document> = Vec::new();

                while let Some(doc) = cur.next().await {
                    if let Ok(d) = doc {
                        vec_doc.push(d);
                    }
                }
        
                Some(vec_doc)
            }
            Err(_) => {
                return None;
            }
        }
    }

}
