
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use crate::{structs::users::CustomUser, clients::shopify::types::{Product as ShopifyProduct, Shopify}};

use super::{inputs::{UserId, UserInput, SyncShopifyProductsInput}};





pub struct Query;

#[Object(extends)]
impl Query {


    async fn sync_shopify_products(&self, _ctx: &Context<'_>, args:SyncShopifyProductsInput ) -> FieldResult<Vec<serde_json::Value>> {

    let shopify = Shopify::new(&args.shop, &args.api_key, None);
    let products = shopify.sync_products().await.unwrap();
        Ok(products)
    }

}




pub struct Mutation;

#[Object]
impl Mutation {
    //user mutation
    async fn create_user(&self, _ctx: &Context<'_>, _args: UserInput) -> FieldResult<CustomUser> {
     let user = CustomUser {
            id: "sad".to_string(),
            name: "sad".to_string(),
            email: "sad".to_string(),
            phone: "sad".to_string(),
        };
        Ok(user)
    }

}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
