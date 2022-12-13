use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type Seasons = Vec<SeasonData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonData {
    pub id: i64,
    pub url: String,
    pub number: i64,
    pub name: String,
    pub episode_order: Option<i64>,
    pub premiere_date: String,
    pub end_date: String,
    pub network: Network,
    pub web_channel: Value,
    pub image: Option<Image>,
    pub summary: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: i64,
    pub name: String,
    pub country: Country,
    pub official_site: Option<String>,
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
pub struct Image {
    pub medium: Option<String>,
    pub original: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}
