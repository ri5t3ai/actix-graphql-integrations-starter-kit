use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharlesOrder {
  pub id: String,
  pub amount: String,
  pub code: String,

  pub currency: String,
}