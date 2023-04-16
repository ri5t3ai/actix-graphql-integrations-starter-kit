use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomOrder {
  pub amount: String,
  pub code: String,
  #[serde(rename = "type")]
  pub type_: String,
}