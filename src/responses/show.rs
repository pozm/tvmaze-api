use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowData {
    pub id: i64,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub language: String,
    pub genres: Vec<String>,
    pub status: String,
    pub runtime: i64,
    pub average_runtime: i64,
    pub premiered: String,
    pub ended: Value,
    pub official_site: Value,
    pub schedule: Schedule,
    pub rating: Rating,
    pub weight: i64,
    pub network: Network,
    pub web_channel: Value,
    pub dvd_country: Value,
    pub externals: Externals,
    pub image: Image,
    pub summary: String,
    pub updated: i64,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub time: String,
    pub days: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub average: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: i64,
    pub name: String,
    pub country: Country,
    pub official_site: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub name: String,
    pub code: String,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Externals {
    pub tvrage: Value,
    pub thetvdb: i64,
    pub imdb: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub medium: String,
    pub original: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Self_field,
    pub previousepisode: Previousepisode,
    pub nextepisode: Nextepisode,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Previousepisode {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nextepisode {
    pub href: String,
}
