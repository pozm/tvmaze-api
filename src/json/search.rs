use serde::{Serialize, Deserialize};

use super::show::Show;

pub type Root = Vec<RootItem>;

#[derive(Deserialize, Debug, Serialize,Clone)]
pub struct RootItem {
    pub score: f32,
    pub show: Show,
}