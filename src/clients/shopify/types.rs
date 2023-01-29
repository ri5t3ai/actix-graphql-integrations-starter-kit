use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
#[derive(Clone, Debug)]
pub struct Shopify {
    pub shared_secret: Option<String>,
    pub api_key: String,
    pub query_url: String,
    pub rest_url: String,
    pub shop: String,
}

#[derive(Clone, Debug)]
pub enum ShopifyAPIError {
    ConnectionFailed,
    ResponseBroken,
    NotJson,
    NotWantedJsonFormat,
    Throttled,
}

#[derive( Deserialize)]
pub struct Shop {
 pub id: i64,
  pub address1: String,
  pub address2: String,
  pub city: String,
  pub country: String,
  pub country_code: String,
  pub country_name: String,
//  created_at: DateTime<Utc>,
//pub    updated_at: DateTime<Utc>,
  pub customer_email: String,
  pub currency: String,
  pub domain: String,
  pub email: String,
  pub google_apps_domain: Option<String>,
  pub google_apps_login_enabled: Option<bool>,
  pub latitude: f32,
  pub longitude: f32,
  pub money_format: String,
  pub money_with_currency_format: String,
  pub weight_unit: String,
  pub myshopify_domain: String,
  pub name: String,
  pub plan_name: String,
  pub has_discounts: bool,
  pub has_gift_cards: bool,
  pub plan_display_name: String,
  pub password_enabled: bool,
  pub phone: Option<String>,
  pub primary_locale: String,
  pub province: String,
  pub province_code: Option<String>,
  pub shop_owner: String,
  pub source: Option<String>,
  pub force_ssl: bool,
  pub tax_shipping: Option<bool>,
  pub taxes_included: Option<bool>,
  pub county_taxes: Option<bool>,
  pub timezone: String,
  pub iana_timezone: String,
  pub zip: String,
  pub has_storefront: bool,
  pub setup_required: bool,
  pub primary_location_id: Value,
  pub money_in_emails_format: Value,
  pub money_with_currency_in_emails_format: Value,
  pub eligible_for_payments: Value,
  pub requires_extra_payments_agreement: Value,
  pub eligible_for_card_reader_giveaway: Value,
  pub finances: Value,
}

#[derive( Serialize, Deserialize)]
pub struct Location {
  pub id: i64,
  pub name: String,
  pub address1: Option<String>,
  pub address2: Option<String>,
  pub city: Option<String>,
  pub zip: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub phone: Option<String>,
  pub country_code: Option<String>,
  pub country_name: Option<String>,
  pub province_code: Option<String>,
  pub legacy: bool,
  pub active: bool,
  pub created_at: String,
  pub updated_at: String,
}

#[derive( Serialize, Deserialize)]
pub struct InventoryLevel {
  pub inventory_item_id: i64,
  pub location_id: i64,
  pub available: Value,
  pub admin_graphql_api_id: String,
//  pub updated_at: DateTime<Utc>,
}


#[derive( Serialize, Deserialize)]
pub struct Variant {
  pub id: i64,
  pub product_id: i64,
  pub title: String,
  pub price: String,
  pub sku: String,
  pub position: i64,
  pub inventory_policy: String,
  pub compare_at_price: Option<String>,
  pub fulfillment_service: Option<String>,
  pub inventory_management: Option<String>,
  pub option1: Option<String>,
  pub option2: Option<String>,
  pub option3: Option<String>,
//  pub created_at: DateTime<Utc>,
//  pub updated_at: DateTime<Utc>,
  pub taxable: bool,
  pub barcode: Option<String>,
  pub grams: i64,
  pub image_id: Option<i64>,
  pub inventory_quantity: i64,
  pub weight: f64,
  pub weight_unit: String,
  pub inventory_item_id: i64,
  pub old_inventory_quantity: i64,
  pub requires_shipping: bool,
}

#[derive( Serialize, Deserialize)]
pub struct Product {
  pub id: i64,
  pub title: String,
  pub body_html: Option<String>,
  pub vendor: String,
  pub product_type: String,
//  pub created_at: DateTime<Utc>,
  pub handle: String,
//  pub updated_at: Option<DateTime<Utc>>,
//  pub published_at: Option<DateTime<Utc>>,
  pub template_suffix: Option<String>,
  pub tags: String,
  pub published_scope: String,
  pub variants: Vec<Variant>,
  pub options: Vec<ProductOption>,
  pub images: Vec<Image>,
  pub image: Option<Image>,
}

#[derive( Serialize, Deserialize)]
pub struct Image {
  pub id: i64,
  pub product_id: i64,
  pub position: i64,
  pub created_at: String,
//  pub updated_at: Option<DateTime<Utc>>,
  pub alt: Option<String>,
  pub width: i64,
  pub height: i64,
  pub src: String,
  pub variant_ids: Vec<i64>,
}

#[derive( Serialize, Deserialize)]
pub struct ProductOption {
  pub id: i64,
  pub product_id: i64,
  pub name: String,
  pub position: i64,
  pub values: Vec<String>,
}

#[derive( Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentStatus {
  Fulfilled,
  Partial,
}

#[derive( Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinancialStatus {
  Pending,
  Authorized,
  PartiallyPaid,
  Paid,
  PartiallyRefunded,
  Refunded,
  Voided,
}

#[derive( Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShipmentStatus {
  Confirmed,
  InTransit,
  OutForDelivery,
  AttemptedDelivery,
  Delivered,
  Failure,
  LabelPrinted,
}

#[derive( Serialize, Deserialize)]
pub struct Address {
  pub first_name: Option<String>,
  pub address1: String,
  pub phone: Option<String>,
  pub city: String,
  pub zip: Option<String>,
  pub province: Option<String>,
  pub country: String,
  pub last_name: Option<String>,
  pub address2: Option<String>,
  pub company: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub name: String,
  pub country_code: Option<String>,
  pub province_code: Option<String>,
}

#[derive( Serialize, Deserialize)]
pub struct ClientDetails {
  browser_ip: Option<String>,
  accept_language: Option<String>,
  user_agent: Option<String>,
  session_hash: Option<Value>,
  browser_width: Option<i64>,
  browser_height: Option<i64>,
}

#[derive( Serialize, Deserialize)]
pub struct Customer {
  pub id: i64,
  pub email: Option<String>,
  pub accepts_marketing: bool,
  pub created_at: String,
  pub updated_at: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub orders_count: i64,
  pub state: String,
  pub total_spent: String,
  pub last_order_id: Option<i64>,
  pub note: Value,
  pub verified_email: bool,
  pub multipass_identifier: Value,
  pub tax_exempt: bool,
  pub phone: Option<String>,
  pub tags: String,
  pub last_order_name: Option<String>,
  pub default_address: Option<DefaultAddress>,
}

#[derive( Serialize, Deserialize)]
pub struct DefaultAddress {
  pub id: i64,
  pub customer_id: i64,
  pub first_name: Option<String>,
  pub last_name: String,
  pub company: Option<String>,
  pub address1: String,
  pub address2: Option<String>,
  pub city: String,
  pub province: Option<String>,
  pub country: String,
  pub zip: Option<String>,
  pub phone: Option<String>,
  pub name: String,
  pub province_code: Option<String>,
  pub country_code: String,
  pub country_name: String,
  pub default: bool,
}

#[derive( Serialize, Deserialize)]
pub struct Property {
  pub name: String,
  pub value: Option<Value>,
}

#[derive( Serialize, Deserialize)]
pub struct LineItems {
  pub id: i64,
  pub variant_id: Option<i64>,
  pub title: String,
  pub quantity: i64,
  pub price: String,
  pub grams: i64,
  pub sku: Option<String>,
  pub variant_title: Option<String>,
  pub vendor: Option<String>,
  pub fulfillment_service: String,
  pub product_id: Option<i64>,
  pub requires_shipping: bool,
  pub taxable: bool,
  pub gift_card: bool,
  pub name: String,
  pub variant_inventory_management: Option<String>,
  pub properties: Vec<Property>,
  pub product_exists: bool,
  pub fulfillable_quantity: i64,
  pub total_discount: String,
  pub fulfillment_status: Option<FulfillmentStatus>,
  pub tax_lines: Vec<TaxLines>,
  pub origin_location: Option<Location>,
  pub destination_location: Option<Location>,
}


#[derive( Serialize, Deserialize)]
pub struct DiscountCode {
  pub amount: String,
  pub code: String,
  #[serde(rename = "type")]
  pub type_: String,
}

#[derive( Serialize, Deserialize)]
pub struct Order {
  pub id: i64,
  pub email: Option<String>,
//  pub closed_at: Option<DateTime<Utc>>,
//  pub created_at: DateTime<Utc>,
//  pub updated_at: DateTime<Utc>,
  pub number: i64,
  pub note: Option<String>,
  pub token: String,
  pub gateway: Option<String>,
  pub test: bool,
  pub total_price: String,
  pub subtotal_price: String,
  pub total_weight: Option<i64>,
  pub total_tax: String,
  pub taxes_included: bool,
  pub currency: String,
  pub financial_status: FinancialStatus,
  pub confirmed: bool,
  pub total_discounts: String,
  pub total_line_items_price: String,
  pub cart_token: Option<String>,
  pub buyer_accepts_marketing: bool,
  pub name: String,
  pub referring_site: Value,
  pub landing_site: Value,
//   pub cancelled_at: Option<DateTime<Utc>>,
  pub cancel_reason: Option<String>,
  pub total_price_usd: String,
  pub checkout_token: Value,
  pub reference: Value,
  pub user_id: Option<i64>,
  pub location_id: Option<i64>,
  pub source_identifier: Value,
  pub source_url: Option<String>,
  pub processed_at: String,
  pub device_id: Value,
  pub phone: Option<String>,
  pub customer_locale: Option<String>,
  pub app_id: i64,
  pub browser_ip: Option<String>,
  pub landing_site_ref: Value,
  pub order_number: i64,
  pub discount_codes: Vec<DiscountCode>,
  pub note_attributes: Vec<Property>,
  pub payment_gateway_names: Vec<String>,
  pub processing_method: String,
  pub checkout_id: Value,
  pub source_name: String,
  pub fulfillment_status: Option<FulfillmentStatus>,
  pub tax_lines: Vec<TaxLines>,
  pub tags: String,
  pub contact_email: Option<String>,
  pub order_status_url: String,
  pub line_items: Vec<LineItems>,
  pub shipping_lines: Vec<ShippingLines>,
  pub billing_address: Option<Address>,
  pub shipping_address: Option<Address>,
  pub fulfillments: Vec<Fulfillment>,
  pub client_details: Option<ClientDetails>,
  pub refunds: Vec<Value>,
  pub customer: Option<Customer>,
}

#[derive( Serialize, Deserialize)]
pub struct Fulfillment {
  pub id: i64,
  pub order_id: i64,
  pub status: String,
  pub created_at: String,
  pub service: String,
  pub updated_at: String,
  pub tracking_company: Option<String>,
  pub shipment_status: Option<ShipmentStatus>,
  pub tracking_number: Option<String>,
  pub tracking_numbers: Vec<String>,
  pub tracking_url: Option<String>,
  pub tracking_urls: Vec<String>,
  pub receipt: Value,
  pub line_items: Vec<LineItems>,
}

#[derive( Serialize, Deserialize)]
pub struct ShippingLines {
  pub id: i64,
  pub title: String,
  pub price: String,
  pub code: String,
  pub source: Option<String>,
  pub phone: Option<String>,
  pub requested_fulfillment_service_id: Value,
  pub delivery_category: Value,
  pub carrier_identifier: Value,
  pub discounted_price: String,
  pub tax_lines: Vec<TaxLines>,
}

#[derive( Serialize, Deserialize)]
pub struct TaxLines {
  pub title: String,
  pub price: String,
  pub rate: f64,
}

#[derive( Serialize, Deserialize)]
pub struct OrderRisk {
  pub cause_cancel: Option<bool>,
  pub checkout_id: Option<i64>,
  pub display: Option<bool>,
  pub id: Option<i64>,
  pub message: String,
  pub order_id: Option<i64>,
  pub recommendation: String,
  pub score: Option<String>,
  pub source: String,
}

#[derive( Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentServiceScope {
  CurrentClient,
  All,
}

impl AsRef<str> for FulfillmentServiceScope {
  fn as_ref(&self) -> &str {
    match *self {
      FulfillmentServiceScope::CurrentClient => "current_client",
      FulfillmentServiceScope::All => "all",
    }
  }
}

#[derive( Deserialize)]
pub struct FulfillmentService {
  pub id: i64,
  pub name: String,
  pub handle: String,
  pub email: Option<String>,
  pub include_pending_stock: bool,
  pub service_name: String,
  pub inventory_management: bool,
  pub tracking_support: bool,
  pub provider_id: Option<i64>,
  pub location_id: i64,
}

#[derive( Serialize)]
pub struct NewFulfillmentService {
  pub name: String,
  pub callback_url: String,
  pub inventory_management: bool,
  pub tracking_support: bool,
  pub requires_shipping_method: bool,
  pub format: String,
}

#[derive( Serialize, Default)]
pub struct UpdateFulfillmentService {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub callback_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inventory_management: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tracking_support: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub requires_shipping_method: Option<bool>,
}

