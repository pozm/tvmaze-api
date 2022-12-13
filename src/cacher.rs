use std::{fmt::Debug, time::SystemTime};

use anyhow::Ok;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

#[async_trait::async_trait]
pub trait TvMazeCacher: Debug {
    async fn get(&self, url: &str) -> Option<Value>;
    async fn set(&mut self, url: &str, data: &Value) -> Option<()>;
}

#[derive(Debug)]
pub struct InMemoryCacher {
    data: std::collections::HashMap<String, (Value,SystemTime)>,
    cache_time: u64,
} 
impl InMemoryCacher {
    /// cache time in seconds
    pub fn new(cache_time:Option<u64>) -> Self {
        Self {
            data: std::collections::HashMap::new(),
            cache_time: cache_time.unwrap_or(60 * 60),
        }
    }
}
#[async_trait::async_trait]
impl TvMazeCacher for InMemoryCacher {
    async fn get(&self, url: &str) -> Option<Value> {
        if let Some(ref da ) = self.data.get(url) {
            if da.1.elapsed().unwrap().as_secs() > self.cache_time {
                return None;
            }
            Some(da.0.clone())
        } else {
            None

        }
    }
    async fn set(&mut self, url: &str, data: &Value) -> Option<()> {
        self.data.insert(url.to_string(), (data.clone(), SystemTime::now()));
        Some(())
    }
}