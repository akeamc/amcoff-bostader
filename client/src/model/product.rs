use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use time::Date;

use crate::{
    model::yyyy_mm_dd, Address, Priority, Property, PropertyDetail, QueuePosition, Worker,
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Product {
    #[serde_as(as = "DisplayFromStr")]
    pub product_id: u32,
    #[serde(rename = "type")]
    pub typ: String,
    pub description: String,
    pub short_description: String,
    pub area: String,
    #[serde(rename = "objectnumber")]
    pub object_number: String,
    pub lghnummer: String,
    pub address: String,
    pub zipcode: String,
    pub city: String,
    #[serde_as(as = "DisplayFromStr")]
    pub floor: i8,
    #[serde_as(as = "DisplayFromStr")]
    pub sqr_mtrs: f32,
    #[serde_as(as = "DisplayFromStr")]
    pub reserved: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub number_of_reservations: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub queue_number: u32,
    #[serde(with = "yyyy_mm_dd")]
    pub move_in_date: Date,
    #[serde(with = "yyyy_mm_dd")]
    pub reserve_from_date: Date,
    #[serde(with = "yyyy_mm_dd")]
    pub reserve_until_date: Date,
    pub priority: Option<Priority>,
    #[serde_as(as = "DisplayFromStr")]
    pub rent: u32,
}

impl From<Product> for Property {
    fn from(p: Product) -> Self {
        Self {
            id: p.product_id,
            description: p.description,
            short_description: p.short_description,
            address: Address {
                street: p.address,
                city: p.city,
                postal_code: p.zipcode,
            },
            property_type: p.typ.parse().unwrap(),
            area: p.area,
            queue_position: QueuePosition {
                position: p.queue_number,
                total_in_queue: p.number_of_reservations,
            },
            reserved: p.reserved,
            floor: p.floor,
            size_sqm: p.sqr_mtrs,
            reserve_from: p.reserve_from_date,
            reserve_until: p.reserve_until_date,
            move_in: p.move_in_date,
            priority: p.priority,
            rent: p.rent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct HouseCaretaker {
    pub worker_id: String,
    pub login_id: String,
    pub name: String,
    pub phone: String,
    pub workphone: String,
    pub email: String,
}

impl From<HouseCaretaker> for Worker {
    fn from(value: HouseCaretaker) -> Self {
        Self {
            id: value.worker_id,
            email: value.email,
            name: value.name,
            phone: value.phone,
            work_phone: value.workphone,
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct ProductDetail {
    #[serde(flatten)]
    pub product: Product,
    pub status: String,
    #[serde(rename = "storenumber")]
    pub store_number: String,
    pub store_included: String,
    #[serde(rename = "storeaddress")]
    pub store_address: String,
    pub store_size: String,
    pub house_caretaker: HouseCaretaker,
    pub shower: String,
    pub furniture: String,
    #[serde(rename = "balkony")]
    pub balcony: String,
    #[serde(rename = "citchen")]
    pub kitchen: String,
    pub elevator: String,
    pub heating: String,
    pub electricity: String,
    pub internet: String,
    pub location: String,
}

impl From<ProductDetail> for PropertyDetail {
    fn from(p: ProductDetail) -> Self {
        Self {
            property: p.product.into(),
            status: p.status.parse().unwrap(),
            store: None,
            caretaker: p.house_caretaker.into(),
            shower: p.shower.parse().unwrap(),
            furniture: p.furniture,
            balcony: p.balcony,
            kitchen: p.kitchen,
            elevator: p.elevator,
            heating: p.heating,
            electricity: p.electricity,
            internet: p.internet,
            facing: p.location,
        }
    }
}
