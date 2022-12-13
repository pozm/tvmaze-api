pub mod cacher;
pub mod episode;
pub mod season;
pub mod show;
use std::{fmt::Debug, sync::RwLock};

use anyhow::Result;
use cacher::TvMazeCacher;
use responses::show::ShowData;
use serde::de::DeserializeOwned;
use serde_json::Value;
use show::Show;
pub mod responses;
#[derive(Debug)]

pub struct TvMazeClient {
    pub(crate) client: reqwest::Client,
    pub cacher: RwLock<Box<dyn cacher::TvMazeCacher>>,

}


pub enum ShowLookup<'a> {
    TVDB(u32),
    IMDB(&'a str),
}

impl TvMazeClient {
    const BASE_URL: &'static str = "http://api.tvmaze.com";
    pub fn new(cacher : Box<dyn TvMazeCacher>) -> Self {
        Self {
            client: reqwest::Client::new(),
            // rwlock cause i really don't want to have to change everything to be &mut self, lol.
            cacher: RwLock::new(cacher),
        }
    }
    pub async fn fetch<T:DeserializeOwned>(&self, url: &str) -> Result<T> {
        if let Some(data) = self.cacher.read().unwrap().get(url).await {
            println!("Cache hit for {}", url);
            return Ok(serde_json::from_value(data)?);
        }
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Status was not success: {}", response.status()));
        }
        let data = response.json::<Value>().await?;
        self.cacher.write().unwrap().set(url, &data).await;
        Ok(serde_json::from_value(data)?)
    }

    pub async fn lookup_show<'a>(&self,lookup_type: ShowLookup<'a>) -> Result<Show> {
        let url = match lookup_type {
            ShowLookup::TVDB(id) => format!("{}/lookup/shows?thetvdb={}", Self::BASE_URL, id),
            ShowLookup::IMDB(id) => format!("{}/lookup/shows?imdb={}", Self::BASE_URL, id),
        };
        let data = self.fetch(&url).await?;
        Ok(Show::new(self, data))
    }
    pub async fn get_show<'a>(&self,id: u32) -> Result<Show> {

        let data = self.fetch(&format!("{}/shows/{}",TvMazeClient::BASE_URL, id)).await?;
        Ok(Show::new(self, data))
    }
}