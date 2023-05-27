use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::common::{Rating, Image, Schedule, Network, Country, Externals};

#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShowEpisode {
    id : i32,
    url : String,
    name : String,
    season : Option<i32>,
    number : Option<i32>,
    #[serde(rename = "type")]
    type_field : String,
    airdate : NaiveDate,
    airtime : String,
    airstamp : DateTime<Utc>,
    runtime : Option<i32>,
    rating : Rating,
    image : Option<Image>,
    summary : Option<String>,
}