use async_graphql::{ SimpleObject};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CharlesUser {
  pub id: String,
  pub name: String,
  pub email: String,
  pub phone: String,
}