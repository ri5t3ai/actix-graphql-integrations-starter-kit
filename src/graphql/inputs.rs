use async_graphql::{Enum, InputObject};

use serde::{Deserialize, Serialize};

#[derive(InputObject)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct UserId {
    pub _id: String,
}


#[derive(InputObject)]
pub struct SyncShopifyProductsInput {
    pub shop: String,
    pub api_key: String,
}


//project schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

