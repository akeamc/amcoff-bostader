use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Picture {
    #[serde_as(as = "DisplayFromStr")]
    pub url: Url,
    pub alt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDetail {
    pub pictures: Vec<Picture>,
}
