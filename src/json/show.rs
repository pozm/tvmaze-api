use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::common::{Rating, Image, Schedule, Network, Country, Externals};


#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]

pub struct Show {
    id : i32,
    url : String,
    name : String,
    #[serde(rename = "type")]
    type_field : String,
    language : String,
    genres : Vec<String>,
    status : String,
    average_runtime : Option<i32>,
    premiered : Option<NaiveDate>,
    ended : Option<NaiveDate>,
    official_site : Option<String>,
    schedule : Schedule,
    rating : Rating,
    weight : i32,
    network : Option<Network>,
    web_channel : Option<Network>,
    dbd_country : Option<Country>,
    externals : Externals,
    image : Option<Image>,
    summary : String,
    updated : i64,
    
}
