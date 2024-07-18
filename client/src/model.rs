time::serde::format_description!(yyyy_mm_dd, Date, "[year]-[month]-[day]");

mod area;
mod product;
mod property;
mod user;

pub use area::*;
pub use product::*;
pub use property::*;
pub use user::*;

#[cfg(test)]
mod tests {
    use crate::{Product, ProductDetail, PropertyDetail};

    #[test]
    fn parse_product() {
        let json = include_bytes!("product.json");
        let _: Product = serde_json::from_slice(json).unwrap();
    }

    #[test]
    fn parse_product_detail() {
        let json = include_bytes!("productDetail.json");
        let p: ProductDetail = serde_json::from_slice(json).unwrap();

        println!(
            "{}",
            serde_json::to_string(&PropertyDetail::from(p)).unwrap()
        );
    }
}
