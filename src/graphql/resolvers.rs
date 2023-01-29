
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use crate::structs::users::CharlesUser;

use super::{inputs::{UserId, UserInput}};

pub struct Query;

#[Object(extends)]
impl Query {
    //users query
    async fn user(&self, _ctx: &Context<'_>, _args: UserId) -> FieldResult<CharlesUser> {

        let user = CharlesUser {
            id: "sad".to_string(),
            name: "sad".to_string(),
            email: "sad".to_string(),
            phone: "sad".to_string(),
        };

        Ok(user)
    }
    async fn authenticate(&self, _ctx: &Context<'_>, _args: UserId) -> FieldResult<CharlesUser> {

        let user = CharlesUser {
            id: "sad".to_string(),
            name: "sad".to_string(),
            email: "sad".to_string(),
            phone: "sad".to_string(),
        };

        Ok(user)
    }

}




pub struct Mutation;

#[Object]
impl Mutation {
    //user mutation
    async fn create_user(&self, _ctx: &Context<'_>, _args: UserInput) -> FieldResult<CharlesUser> {
     let user = CharlesUser {
            id: "sad".to_string(),
            name: "sad".to_string(),
            email: "sad".to_string(),
            phone: "sad".to_string(),
        };
        Ok(user)
    }

}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
