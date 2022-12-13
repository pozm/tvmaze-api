use serde::Deserialize;
use serde::Serialize;

pub type Episodes = Vec<EpisodeData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeData {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub season: i64,
    pub number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub airdate: String,
    pub airtime: String,
    pub airstamp: String,
    pub runtime: i64,
    pub rating: Rating,
    pub image: Option<Image>,
    pub summary: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub average: Option<f64>,
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
    pub show: Show,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub href: String,
}
