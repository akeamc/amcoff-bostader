use core::{convert::Infallible, str::FromStr};

use serde::{Deserialize, Serialize};
use time::Date;

use crate::model::yyyy_mm_dd;

pub type PropertyId = u32;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Apartment,
    CorridorRoom,
    Other(String),
}

impl FromStr for PropertyType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Lägenhet" => Self::Apartment,
            "Korridorrum" => Self::CorridorRoom,
            s => Self::Other(s.to_owned()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Novisch,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    pub id: PropertyId,
    pub description: String,
    pub short_description: String,
    pub address: Address,
    pub property_type: PropertyType,
    pub area: String,
    pub queue_position: QueuePosition,
    pub reserved: bool,
    pub floor: i8,
    pub size_sqm: f32,
    #[serde(with = "yyyy_mm_dd")]
    pub reserve_from: Date,
    #[serde(with = "yyyy_mm_dd")]
    pub reserve_until: Date,
    #[serde(with = "yyyy_mm_dd")]
    pub move_in: Date,
    pub priority: Option<Priority>,
    pub rent: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub postal_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct QueuePosition {
    pub position: u32,
    pub total_in_queue: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyStatus {
    Canceled(Date),
    #[serde(untagged)]
    Other(String),
}

impl FromStr for PropertyStatus {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            s => Self::Other(s.to_owned()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Store {
    pub included: bool,
    pub address: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worker {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub work_phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Shower {
    Own,
    Shared,
}

impl FromStr for Shower {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Egen dusch" => Self::Own,
            _ => unimplemented!(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyDetail {
    #[serde(flatten)]
    pub property: Property,
    pub status: PropertyStatus,
    pub store: Option<Store>,
    pub caretaker: Worker,
    pub shower: Shower,
    pub furniture: String,
    pub balcony: String,
    pub kitchen: String,
    pub elevator: String,
    pub heating: String,
    pub electricity: String,
    pub internet: String,
    pub facing: String,
}
