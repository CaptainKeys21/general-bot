use urlencoding::encode;
use futures::{StreamExt};
use mongodb::{
    bson::{Document, doc},
    error:: Error,
    Client,
    Collection, options::{ FindOneAndUpdateOptions, ReturnDocument, FindOneOptions }, results::UpdateResult,
};
use serde::de::DeserializeOwned;


#[derive(Clone)]
pub struct Mongodb {
    client: Client,
}

impl Mongodb {
    pub async fn new(uri: String) -> Result<Mongodb, Error> {
        let new_client = Client::with_uri_str(uri).await?;

        Ok(Mongodb {
            client: new_client
        })
    }

    pub fn make_uri(username: &str, password: &str, domain: &str, port: &str) -> String {
        format!("mongodb://{}:{}@{}:{}", username, encode(password), domain, port)
    }

    pub async fn insert_one(&self, database: &str, collection: &str, data: Document) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);


        if let Err(e) = collection.insert_one(data, None).await {
            return Err(e);
        }

        Ok(())
    }

    pub async fn update_one<T: DeserializeOwned>(&self, database: &str, collection: &str, query: Document, update: Document) -> Result<Option<T>, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<T>(collection);

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

        collection.insert_many(data, None).await?;

        Ok(())
    }

    pub async fn update_many(&self, database: &str, collection: &str, query: Document, update: Document) -> Result<UpdateResult, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        let result = collection.update_many(query, update, None).await?;

        Ok(result)
    }

    pub async fn get_one<T: DeserializeOwned + Unpin + Send + Sync>(&self, database: &str, collection: &str, filter: Document, options: Option<FindOneOptions>) -> Result<Option<T>, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<T>(collection);

        collection.find_one(filter, options).await
    }

    pub async fn get_collection<T>(&self, database: &str, collection: &str) -> Collection<T> {
        let db = self.client.database(database);

        db.collection::<T>(collection)
    }

    pub async fn get<T: DeserializeOwned + Unpin + Send + Sync>(&self, database: &str, collection: &str, filter: Document) -> Result<Vec<T>, Error>{
        let db = self.client.database(database);
        let collection = db.collection::<T>(collection);


        let mut cursor = collection.find(filter, None).await?;


        let mut vec_doc: Vec<T> = Vec::new();

        while let Some(doc) = cursor.next().await {
            if let Ok(d) = doc {
                vec_doc.push(d);
            }
        }
            
        Ok(vec_doc)
    }

    pub async fn delete_one(&self, database: &str, collection: &str, filter: Document) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        collection.delete_one(filter, None).await?;

        Ok(())
    }

    pub async fn clear_collection(&self, database: &str, collection: &str) -> Result<(), Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        collection.delete_many(doc! {}, None).await?;

        Ok(())
    }

    pub async fn count_collection_data(&self, database: &str, collection: &str, filter: Option<Document>) -> Result<u64, Error> {
        let db = self.client.database(database);
        let collection = db.collection::<Document>(collection);

        let result = collection.count_documents(filter, None).await?;

        Ok(result)
    }

}
