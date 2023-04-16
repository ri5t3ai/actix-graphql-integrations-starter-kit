use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProduct {
  pub id: String,
  pub platform: String,
  pub title: String,
  pub description: String,
  pub price: f32,
  pub quantity: i64,
  #[serde(rename = "type")]
  pub type_: String,
}