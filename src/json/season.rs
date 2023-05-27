use chrono::{NaiveDate, DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::{Rating, Image, Network};

#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShowSeason {
    id : i32,
    url : String,
    name : String,
    number : Option<i32>,
    episode_order : Option<i32>,
    #[serde(rename = "type")]
    premiere_date : NaiveDate,
    end_date : NaiveDate,
    network : Option<Network>,
    web_channel : Option<Network>,
    image : Option<Image>,
    summary : Option<String>,
}