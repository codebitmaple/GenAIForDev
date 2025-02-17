extern crate redis;

use log::{debug, info};
use redis::{Commands, FromRedisValue, ToRedisArgs};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::sync::{Arc, Mutex};

use crate::shared::ops::environ_ops::{Environ, RedisConfig};

#[derive(Clone)]
pub struct Cache {
    redis_conn: Arc<Mutex<redis::Connection>>, // Reuse the connection
    cache_key_prefix: String,
}

impl Default for Cache {
    fn default() -> Self {
        let redis_config: RedisConfig = Environ::init();
        let connection_str = redis_config.redis_server.clone();
        println!("Connecting to Redis at: {}", connection_str);
        let client = match redis::Client::open(connection_str.clone()) {
            Ok(c) => c,
            Err(e) => {
                panic!("Error creating Redis client at {}: {:?}", connection_str, e)
            }
        };
        let conn = match client.get_connection() {
            Ok(c) => c,
            Err(e) => {
                panic!("Error connecting to Redis: {:?}", e)
            }
        };
        Cache {
            redis_conn: Arc::new(Mutex::new(conn)), // Store the connection in an Arc<Mutex<>> for sharing and safety
            cache_key_prefix: redis_config.redis_key_prefix.clone(),
        }
    }
}

impl Cache {
    fn build_key(
        &self,
        key: &str,
    ) -> String {
        format!("{}{}", self.cache_key_prefix, key)
    }

    pub fn get_scalar<T: FromRedisValue>(
        &self,
        key: &str,
    ) -> Option<T> {
        let mut conn = match self.redis_conn.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("Error getting Redis connection: {:?}", e);
                return None;
            }
        }; // Reuse the connection
        let redis_key = self.build_key(key);
        debug!("Getting key: {}", redis_key);
        match conn.get(redis_key) {
            Ok(value) => Some(value),
            Err(e) => {
                println!("Error getting value from Redis: {:?}", e);
                None
            }
        }
    }

    // Removed the specific set_scalar method for String values

    pub fn set_scalar<T: ToRedisArgs>(
        &self,
        key: &str,
        value: T,
    ) {
        let mut conn = match self.redis_conn.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("Error getting Redis connection: {:?}", e);
                return;
            }
        }; // Reuse the connection
        let redis_key = self.build_key(key);
        let _: () = conn.set::<String, T, ()>(redis_key, value).unwrap();
    }

    pub fn set_list<T: Serialize>(
        &self,
        key: &str,
        value: &Vec<T>,
    ) -> bool {
        let mut conn = match self.redis_conn.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("Error getting Redis connection: {:?}", e);
                return false;
            }
        };
        let redis_key = self.build_key(key);
        info!("Setting list key: {}", key);
        let _: () = conn.del(redis_key.clone()).unwrap(); // Clear the list
        for item in value {
            if let Ok(json_str) = serde_json::to_string(item) {
                let _: () = conn.rpush(redis_key.clone(), json_str).unwrap();
            }
        }
        true
    }

    pub fn get_range<T: DeserializeOwned>(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Option<Vec<T>> {
        info!("Getting range for key: {}", key);
        let mut conn = match self.redis_conn.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("Error getting Redis connection: {:?}", e);
                return None;
            }
        }; // Reuse the connection
        let redis_key = self.build_key(key);
        debug!("Using Redis key: {}", redis_key);

        if let Ok(json_strs) = conn.lrange::<_, Vec<String>>(redis_key, start, stop) {
            let mut items: Vec<T> = vec![];
            for json_str in json_strs {
                if let Ok(item) = serde_json::from_str(&json_str) {
                    items.push(item);
                }
            }
            if items.is_empty() {
                None
            } else {
                Some(items)
            }
        } else {
            None
        }
    }

    pub fn remove(
        &self,
        key: &str,
    ) -> bool {
        info!("Removing key: {}", key);
        let mut conn = match self.redis_conn.lock() {
            Ok(c) => c,
            Err(e) => {
                println!("Error getting Redis connection: {:?}", e);
                return false;
            }
        }; // Reuse the connection
        let redis_key = self.build_key(key);
        conn.del(redis_key).unwrap_or(0) > 0
    }
}
