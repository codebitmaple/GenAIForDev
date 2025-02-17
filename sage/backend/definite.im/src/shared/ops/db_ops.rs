use std::fmt::{Debug, Error};

use futures::stream::TryStreamExt;
use log::{debug, error};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Client, Collection,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::shared::ops::environ_ops::{DatabaseConfig, Environ};

pub struct Database;

impl Database {
    pub fn generate_id() -> ObjectId {
        ObjectId::new()
    }

    pub async fn get_client() -> Client {
        let db_config: DatabaseConfig = Environ::init();
        match Client::with_uri_str(db_config.db_connection_string).await {
            Ok(client) => client,
            Err(e) => {
                panic!("Error connecting to database: {}", e);
            }
        }
    }

    pub fn get_collection<T>(
        client: &Client,
        collection_name: &str,
    ) -> Collection<T>
    where
        T: Send + Sync + Unpin + Serialize,
    {
        let db_config: DatabaseConfig = Environ::init();
        client.database(db_config.db_name.as_str()).collection::<T>(collection_name)
    }

    pub async fn create<T>(
        collection: &Collection<T>,
        entity: &T,
    ) -> Option<String>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        match collection.insert_one(entity).await {
            Ok(r) => Some(r.inserted_id.as_object_id().unwrap().to_hex()),
            Err(e) => {
                error!("Error creating document in {}: {}", collection.name(), e);
                None
            }
        }
    }

    pub async fn delete<T>(
        collection: &Collection<T>,
        id: &ObjectId,
    ) -> Option<String>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        // Create a filter to match the document by its _id field
        let filter = doc! { "_id": id };

        // Perform the delete operation
        match collection.delete_one(filter).await {
            Ok(delete_result) => {
                if delete_result.deleted_count == 0 {
                    // No document matched the provided id
                    error!("No document found with _id: {}", id);
                    None
                } else {
                    // Document was successfully deleted
                    Some(id.to_hex())
                }
            }
            Err(e) => {
                error!("Error deleting document in {}: {}", collection.name(), e);
                None
            }
        }
    }

    pub async fn update<T>(
        collection: &Collection<T>,
        id: &ObjectId,
        update_doc: Document,
    ) -> Option<String>
    where
        T: Serialize + Unpin + Send + Sync,
    {
        // Create a filter to match the document by its _id field
        let filter = doc! { "_id": id };

        // Perform the update operation
        match collection.update_one(filter, update_doc).await {
            Ok(update_result) => {
                if update_result.matched_count == 0 {
                    // No document matched the provided id
                    error!("No document found with _id: {}", id);
                    None
                } else {
                    // Document was successfully updated
                    Some(id.to_string())
                }
            }
            Err(e) => {
                error!("Error updating document in {}: {}", collection.name(), e);
                None
            }
        }
    }

    pub async fn scan<T>(
        collection: Collection<T>,
        filter: bson::Document,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned + Unpin + Send + Sync + Debug,
    {
        match collection.find(filter).await {
            Ok(mut cursor) => {
                let mut entities = vec![];
                loop {
                    match cursor.try_next().await {
                        Ok(Some(entity)) => {
                            debug!("Got entity: {:?}", entity);
                            entities.push(entity)
                        }
                        Ok(None) => break,
                        Err(e) => {
                            error!("Error iterating through documents from {}: {:?}", collection.name(), e);
                            return Err(Error);
                        }
                    }
                }
                Ok(entities)
            }
            Err(e) => {
                error!("Error finding documents from {}: {}", collection.name(), e);
                Err(Error)
            }
        }
    }

    pub async fn find<T>(
        collection: Collection<T>,
        id: &ObjectId,
    ) -> Option<T>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        match collection.find_one(doc! {"_id": id}).await {
            Ok(cursor) => match cursor {
                Some(r) => Some(r),
                None => {
                    error!("No document found with id {} in {}", id, collection.name());
                    None
                }
            },
            Err(e) => {
                error!("Error finding document with id {} in {}: {}", id, collection.name(), e);
                None
            }
        }
    }

    pub async fn filter<T>(
        collection: Collection<T>,
        filter: Document,
    ) -> Option<T>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        match collection.find_one(filter.clone()).await {
            Ok(cursor) => match cursor {
                Some(r) => Some(r),
                None => {
                    error!("No document found with filter {:?} in {}", filter, collection.name());
                    None
                }
            },
            Err(e) => {
                error!("Error finding document with filter {:?} in {}: {}", filter, collection.name(), e);
                None
            }
        }
    }
}
