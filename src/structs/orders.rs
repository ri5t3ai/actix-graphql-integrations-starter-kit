use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomOrder {
  pub id: String,
  pub amount: String,
  pub code: String,

  pub currency: String,
}