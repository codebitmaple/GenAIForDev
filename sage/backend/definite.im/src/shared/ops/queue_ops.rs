use std::{sync::Arc, thread, time::Duration};

use log::{debug, error};
use redis::{Commands, RedisResult};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct RedisConnectionManager {
    pub client: redis::Client,
}

impl RedisConnectionManager {
    pub fn new(redis_url: &str) -> Self {
        Self {
            client: redis::Client::open(redis_url).expect("Failed to create Redis client"),
        }
    }

    pub fn get_connection(&self) -> RedisResult<redis::Connection> {
        self.client.get_connection()
    }
}

pub struct Producer {
    pub connection_manager: Arc<RedisConnectionManager>,
}

impl Producer {
    pub fn new(connection_manager: Arc<RedisConnectionManager>) -> Self {
        Self { connection_manager }
    }

    // Now accepts serde_json::Value and serializes it to a string before pushing to Redis
    pub fn produce(
        &self,
        queue: &str,
        message: &Value,
    ) -> RedisResult<()> {
        let mut con = self.connection_manager.get_connection()?;
        let message_str = serde_json::to_string(message).expect("Failed to serialize message");
        match con.lpush::<&str, String, i32>(queue, message_str) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to push message to queue: {:?}", e);
                Err(e)
            }
        }
    }
}

pub struct Consumer {
    connection_manager: Arc<RedisConnectionManager>,
}

impl Consumer {
    pub fn new(connection_manager: Arc<RedisConnectionManager>) -> Self {
        Self { connection_manager }
    }

    // Consume messages from multiple queues, with a custom deserialization and processing function
    pub fn consume_multiple_queues<T, F, G>(
        &self,
        queues: &[&str],
        deserialize_fn: F,
        process_fn: G,
    ) -> RedisResult<()>
    where
        T: DeserializeOwned + 'static,
        F: Fn(&str) -> Result<T, serde_json::Error> + Send + Sync + 'static,
        G: Fn(T) -> bool + Send + Sync + 'static,
    {
        let mut con = self.connection_manager.get_connection()?;
        loop {
            let msg: RedisResult<(String, String)> = con.brpop(queues, 0.0); // Block until a message is available
            match msg {
                Ok((queue, message_str)) => {
                    println!("Received message from queue {}: {}", queue, message_str);
                    match deserialize_fn(&message_str) {
                        Ok(message) => {
                            if !process_fn(message) {
                                match con.lpush::<&str, &str, i32>(&queue, &message_str) {
                                    Ok(_) => {
                                        error!("Failed to process message, requeued: {}", message_str);
                                    }
                                    Err(e) => {
                                        error!("Failed to requeue message: {:?}", e);
                                    }
                                }
                            } else {
                                match con.lrem::<&str, &str, i32>("processing_queue", 1, &message_str) {
                                    Ok(_) => debug!("Message processed successfully: {}", message_str),
                                    Err(e) => debug!("Failed to remove message from processing queue: {:?}", e),
                                } // Remove successfully processed message
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize message: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message from queues: {:?}", e);
                }
            }
            thread::sleep(Duration::from_secs(1)); // Simulate processing delay
        }
    }
}
