use log::{error, info};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

use crate::shared::models::google::GoogleUserModel;
use crate::shared::ops::db_ops::Database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
    pub _id: Option<ObjectId>,
    pub google_id: String,
    pub email: String,
    pub verified_email: bool,
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            _id: None,
            google_id: "not-set".to_string(),
            email: "not-set".to_string(),
            verified_email: false,
            full_name: "not-set".to_string(),
            given_name: "not-set".to_string(),
            family_name: "not-set".to_string(),
            picture: "not-set".to_string(),
        }
    }
}

impl UserEntity {
    pub fn get_collection(mongoc: &Client) -> Collection<UserEntity> {
        Database::get_collection::<UserEntity>(mongoc, "users")
    }

    pub fn from(user: GoogleUserModel) -> UserEntity {
        UserEntity {
            _id: ObjectId::new().into(),
            google_id: user.id,
            email: user.email.clone(),
            verified_email: user.verified_email,
            full_name: user.name,
            given_name: user.given_name,
            family_name: user.family_name,
            picture: user.picture,
        }
    }

    pub async fn create(
        &self,
        mongoc: &Client,
    ) -> Option<String> {
        let collection = Self::get_collection(mongoc);
        let user = match Database::filter(collection.clone(), doc! {"google_id": self.google_id.clone()}).await {
            Some(r) => Some(r),
            None => {
                error!("Failed to get user: {:?}", self);
                None
            }
        };

        if user.is_some() {
            info!("User already exists: {:?}", user);
            return Some(user.unwrap()._id.unwrap().to_hex());
        }
        match Database::create(&collection, self).await {
            Some(r) => Some(r),
            None => {
                error!("Failed to create user: {:?}", self);
                None
            }
        }
    }
}
