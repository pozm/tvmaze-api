use std::ops::Deref;

use crate::{responses::episode::EpisodeData, show::Show};


#[derive(Debug)]
pub struct Episode<'a> {
    pub data: EpisodeData,
    pub show: &'a Show<'a>,
    pub client: &'a crate::TvMazeClient,
}
impl Deref for Episode<'_> {
    type Target = EpisodeData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a> Episode<'a> {
    pub fn new(client: &'a crate::TvMazeClient, show: &'a Show, data: EpisodeData) -> Self {
        Self {
            client,
            show,
            data,
        }
    }
    pub fn show(&self) -> &Show {
        self.show
    }
    pub fn client(&self) -> &crate::TvMazeClient {
        self.client
    }
}