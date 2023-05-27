use reqwest::Response;
use thiserror::Error;

use crate::json::{search, show::{Show}, episode::ShowEpisode, season::ShowSeason};


#[derive(Default,Debug,Clone)]
pub struct TvMazeClient {
    inner: reqwest::Client,
}

impl TvMazeClient {
    const BASE_URL: &'static str = "https://api.tvmaze.com";

    async fn req(&self, url: &str) -> Result<Response,TvMazeError> {
        self.inner.get(format!("{}/{url}",Self::BASE_URL)).send().await.map_err(TvMazeError::Reqwest)
    }

    // searching

    pub async fn search(&self, query: &str) -> Result<search::Root,TvMazeError> {
        self.req(&format!("search/shows?q={query}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }
    pub async fn search_single(&self, query: &str) -> Result<Option<Show>,TvMazeError> {
        self.req(&format!("singlesearch/shows?q={query}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }


    // shows

    pub async fn show_lookup(&self,lookup : TvShowLookup) -> Result<Option<Show>,TvMazeError> {
        match lookup {
            TvShowLookup::Tvdb(id) => {
                self.req(&format!("lookup/shows?thetvdb={id}")).await?.json().await.map_err(TvMazeError::Reqwest)
            },
            TvShowLookup::Imdb(id) => {
                self.req(&format!("lookup/shows?imdb={id}")).await?.json().await.map_err(TvMazeError::Reqwest)
            }
        }
    }

    pub async fn show_id(&self, id: i32) -> Result<Show,TvMazeError> {
        self.req(&format!("shows/{id}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }
    pub async fn schedule(&self, country : Option<&str>, date : Option<chrono::NaiveDate>) -> Result<Vec<ShowEpisode>, TvMazeError> {
        let mut req = self.inner.get(format!("{}/schedule/web",Self::BASE_URL));
        let req = if let Some(country) = country {
            req.query(&[("country",country)])
        } else {req};
        let req = if let Some(date) = date {
            req.query(&[("date",date)])
        } else {req};

        req.send().await.map_err(TvMazeError::Reqwest)?.json().await.map_err(TvMazeError::Reqwest)
    }
    pub async fn show_akas(&self, show_id : i32) -> Result<Vec<Show>,TvMazeError> {
        self.req(&format!("shows/{show_id}/akas")).await?.json().await.map_err(TvMazeError::Reqwest)
    }

    // episodes

    pub async fn show_episode_by_index(&self, show_id : i32, season : i32, episode : i32) -> Result<ShowEpisode, TvMazeError> {
        self.req(&format!("shows/{show_id}/episodebynumber?season={season}&number={episode}",show_id=show_id,season=season,episode=episode)).await?.json().await.map_err(TvMazeError::Reqwest)
    }
    pub async fn show_episode_by_date(&self, show_id : i32, date : chrono::NaiveDate) -> Result<ShowEpisode, TvMazeError> {
        self.req(&format!("shows/{show_id}/episodesbydate?date={date}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }
    pub async fn show_episodes_by_season(&self,show_id : i32, season : i32) -> Result<Vec<ShowEpisode>,TvMazeError> {
        self.req(&format!("shows/{show_id}/seasons/{season}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }

    pub async fn episode_index(&self, episode_id : i32) -> Result<ShowEpisode, TvMazeError> {
        self.req(&format!("episodes/{episode_id}")).await?.json().await.map_err(TvMazeError::Reqwest)
    }

    // seasons
    pub async fn show_seasons(&self, show_id : i32) -> Result<Vec<ShowSeason>,TvMazeError> {
        self.req(&format!("shows/{show_id}/seasons")).await?.json().await.map_err(TvMazeError::Reqwest)
    }

}

pub enum TvShowLookup {
    Tvdb(i32),
    Imdb(i32)
}

#[derive(Error,Debug)]
pub enum TvMazeError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}


#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    #[test]
    fn test() {
        use super::*;
        let client = TvMazeClient::default();
        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(client.schedule(None, None)).unwrap();
        println!("{:#?}",res);
    }
}