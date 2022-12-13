use std::ops::Deref;
use anyhow::Result;

use crate::episode::Episode;
use crate::responses::show::ShowData;
use crate::TvMazeClient;
use crate::season::Season;
#[derive(Debug)]
pub struct Show<'a> {
    client: &'a TvMazeClient,
    data : ShowData,
}
impl Deref for Show<'_> {
    type Target = ShowData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> Show<'a> {
    pub fn new(client: &'a TvMazeClient, data: ShowData) -> Self {
        Self {
            client,
            data,
        }
    }
    pub async fn seasons(&self) -> Result<Vec<Season>> {

        let data: Vec<crate::responses::season::SeasonData> = self.client.fetch(&format!("{}/shows/{}/seasons", TvMazeClient::BASE_URL, self.id)).await?;
        Ok(data.into_iter().map(|data| Season::new(self.client,&self, data)).collect())
    }
    pub async fn episodes(&self) -> Result<Vec<Episode>> {
        let data: Vec<crate::responses::episode::EpisodeData> = self.client.fetch(&format!("{}/shows/{}/episodes", TvMazeClient::BASE_URL, self.id)).await?;
        Ok(data.into_iter().map(|data| Episode::new(self.client, &self, data)).collect())
    }
}