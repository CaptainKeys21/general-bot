use bson::to_document;
use futures::{StreamExt};
use mongodb::{
    bson::{Document, doc},
    error:: Error,
    Client,
    Collection, options::{ FindOneAndUpdateOptions, ReturnDocument },
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

    pub async fn update_one(&self, database: &str, collection: &str, query: Document, update: Document) -> Result<Option<Document>, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        let result = collection.find_one_and_update(query, update, None).await?;

        Ok(result)
    }

    pub async fn update_or_insert_one(&self, database: &str, collection: &str, query: Document, update: Document) -> Result<Option<Document>, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);
  
        let options = FindOneAndUpdateOptions::builder().upsert(true).return_document(ReturnDocument::After).build();

        let result = collection.find_one_and_update(query, update, options).await?;

        Ok(result)
    }

    pub async fn insert_many(&self, database: &str, collection: &str, data: Vec<Document>) -> Result<(), Error> {
        if data.is_empty() {
            return Ok(());
        }
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        if let Err(e) = collection.insert_many(data, None).await {
            return Err(e);
        }

        Ok(())
    }

    pub async fn update_many(&self, database: &str, collection: &str, query: Document, update: Document) -> Result<Option<Document>, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        let result = collection.update_many(query, update, None).await?;

        let res_doc = match to_document(&result) {
            Ok(d) => Some(d),
            Err(_) => None
        };

        Ok(res_doc)
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

    pub async fn delete_one(&self, database: &str, collection: &str, filter: Document) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        if let Err(e) = collection.delete_one(filter, None).await {
            return Err(e);
        }

        Ok(())
    }

    pub async fn clear_collection(&self, database: &str, collection: &str) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        if let Err(e) = collection.delete_many(doc! {}, None).await {
            return Err(e)
        };

        Ok(())
    }

}
