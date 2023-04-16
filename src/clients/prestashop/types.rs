use serde::{Deserialize, Serialize};
use async_graphql::{ SimpleObject};


#[derive(Debug, Serialize, Deserialize)]
struct Cart {
    id: Option<i32>,
    id_address_delivery: Option<i32>,
    id_address_invoice: Option<i32>,
    id_currency: Option<i32>,
    id_customer: Option<i32>,
    id_guest: Option<i32>,
    id_lang: Option<i32>,
    id_shop_group: Option<i32>,
    id_shop: Option<i32>,
    id_carrier: Option<i32>,
    recyclable: Option<bool>,
    gift: Option<bool>,
    gift_message: Option<String>,
    mobile_theme: Option<bool>,
    delivery_option: Option<String>,
    secure_key: Option<String>,
    allow_seperated_package: Option<bool>,
    date_add: Option<String>,
    date_upd: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CartAssociations {
    pub cart_rows: Option<Vec<CartRow>>,
}

#[derive(Deserialize, Debug)]
pub struct CartRow {
    pub id: String,
    pub id_product: String,
    pub id_product_attribute: String,
    pub quantity: String,
    pub date_add: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Product {
    pub id: String,
    pub id_manufacturer: String,
    pub id_supplier: String,
    pub id_category_default: String,
    pub new: String,
    pub cache_default_attribute: String,
    pub id_default_image: String,
    pub id_default_combination: String,
    pub id_tax_rules_group: String,
    pub position_in_category: String,
    pub manufacturer_name: String,
    pub quantity: String,
    pub product_type: String,
    pub id_shop_default: String,
    pub reference: String,
    pub supplier_reference: String,
    pub location: String,
    pub width: String,
    pub height: String,
    pub depth: String,
    pub weight: String,
    pub quantity_discount: String,
    pub ean13: String,
    pub isbn: String,
    pub upc: String,
    pub cache_is_pack: String,
    pub cache_has_attachments: String,
    pub is_virtual: String,
    pub state: String,
    pub additional_shipping_cost: String,
    pub customizable: String,
    pub text_fields: String,
    pub uploadable_files: String,
    pub active: String,
    pub redirect_type: String,
    pub id_product_redirected: String,
    pub available_for_order: String,
    pub available_date: String,
    pub condition: String,
    pub show_price: String,
    pub indexed: String,
    pub visibility: String,
    pub advanced_stock_management: String,
    pub date_add: String,
    pub date_upd: String,
    pub pack_stock_type: String,
    pub meta_description: String,
    pub meta_keywords: String,
    pub meta_title: String,
    pub link_rewrite: String,
    pub name: String,
    pub description: String,
    pub description_short: String,
    pub available_now: String,
    pub available_later: String,
    pub associations: ProductAssociations,
}
#[derive(Deserialize, Debug)]
pub struct ProductAssociations {
    pub categories: Option<Vec<Category>>,
    pub images: Option<Vec<Image>>,
    pub combinations: Option<Vec<Combination>>,
    pub product_option_values: Option<Vec<ProductOptionValue>>,
    pub product_features: Option<Vec<ProductFeature>>,
    pub tags: Option<Vec<Tag>>,
    pub stock_availables: Option<Vec<StockAvailable>>,
    pub carriers: Option<Vec<Carrier>>,
    pub supplier: Option<Supplier>,
    pub default_category: Option<Category>,
    pub product_customization_fields: Option<Vec<ProductCustomizationField>>,
    pub product_suppliers: Option<Vec<ProductSupplier>>,
    pub accessories: Option<Vec<Product>>,
    pub product_features_values: Option<Vec<ProductFeatureValue>>,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: String,
    pub id_product: String,
    pub position: String,
    pub cover: String,
}

#[derive(Deserialize, Debug)]
pub struct Combination {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductOptionValue {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductFeature {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct StockAvailable {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Carrier {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Supplier {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductCustomizationField {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductSupplier {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductFeatureValue {
    pub id: String,
}
