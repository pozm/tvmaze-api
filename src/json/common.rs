use chrono::{NaiveDate, NaiveDateTime, DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    time : String,
    days : Vec<String>,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]

pub struct Rating {
    average : Option<f32>,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    id : i32,
    name : String,
    country : Country,
    official_site : Option<String>,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]

pub struct Country {
    name : String,
    code : String,
    timezone : String,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Externals {
    tvrage : Option<i32>,
    thetvdb : Option<i32>,
    imdb : Option<String>,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    medium : Option<String>,
    original : Option<String>,
}
#[derive(Deserialize, Debug, Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Aka {
    name : String,
    country : Country,
}