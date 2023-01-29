
mod transformers;
mod graphql;
mod clients;
mod utils;
mod structs;
//add
use actix_cors::Cors;
use actix_web::{
    guard, http,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use graphql::resolvers::{Mutation, Query, ProjectSchema};

//graphql entry
async fn index(schema: Data<ProjectSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //connect to the data source

    let schema_data = Schema::build(Query, Mutation, EmptySubscription)
        .finish();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://healin.co")
            // allow any port on localhost
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(Data::new(schema_data.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
