use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharlesOrder {
  pub amount: String,
  pub code: String,
  #[serde(rename = "type")]
  pub type_: String,
}