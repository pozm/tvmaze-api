use std::ops::Deref;

use anyhow::Result;

use crate::episode::Episode;
use crate::show::Show;
use crate::TvMazeClient;
use crate::responses::season::SeasonData;

#[derive(Debug)]

pub struct Season<'a> {
    show: &'a Show<'a>,
    client: &'a TvMazeClient,
    data: SeasonData,
}
impl Deref for Season<'_> {
    type Target = SeasonData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a> Season<'a> {
    pub fn new(client: &'a TvMazeClient, show: &'a Show, data: SeasonData) -> Self {
        Self {
            client,
            show,
            data,
        }
    }
    pub fn show(&self) -> &Show {
        self.show
    }
    pub fn client(&self) -> &TvMazeClient {
        self.client
    }
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        let data: Vec<crate::responses::episode::EpisodeData> = self.client.fetch(&format!("{}/seasons/{}/episodes", TvMazeClient::BASE_URL, self.id)).await?;
        Ok(data.into_iter().map(|data| Episode::new(self.client, self.show, data)).collect())
    }
}