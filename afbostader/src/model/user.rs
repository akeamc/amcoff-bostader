use serde::{Deserialize, Serialize};
use time::{format_description::BorrowedFormatItem, macros::format_description, Date};

use crate::Address;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    email: String,
    personal_identity_number: Option<String>,
    first_name: String,
    last_name: String,
    address: FullAddress,
    mobile_phone: String,
    start_year: Option<i32>,
    start_semester: String,
    #[serde(with = "super::yyyy_mm_dd::option")]
    date_of_birth: Option<Date>,
}

fn parse_dob(dob: &str) -> Option<Date> {
    const FMT: &[BorrowedFormatItem] = format_description!("[year][month][day]");

    Date::parse(dob.get(..8)?, FMT).ok()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullAddress {
    street: String,
    city: String,
    postal_code: String,
    county: String,
    country: String,
}

impl From<FullAddress> for Address {
    fn from(
        FullAddress {
            street,
            city,
            postal_code,
            ..
        }: FullAddress,
    ) -> Self {
        Self {
            street,
            city,
            postal_code,
        }
    }
}

impl From<UserInfo> for User {
    fn from(i: UserInfo) -> Self {
        Self {
            email: i.email,
            personal_identity_number: i.personalnumber,
            first_name: i.firstname,
            last_name: i.lastname,
            address: FullAddress {
                street: i.street,
                city: i.city,
                postal_code: i.postalcode,
                county: i.county,
                country: i.country,
            },
            mobile_phone: i.mobilephone,
            start_year: i.startyear.parse().ok(),
            start_semester: i.startsemester,
            date_of_birth: parse_dob(&i.dateofbirth),
        }
    }
}

/// User info as returned by the API.
#[doc(hidden)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub personalnumber: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub street: String,
    pub postalcode: String,
    pub city: String,
    pub county: String,
    pub country: String,
    pub mobilephone: String,
    pub startyear: String,
    pub startsemester: String,
    pub dateofbirth: String,
}
