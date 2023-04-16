
pub mod types;
use reqwest::Response;

use crate::{structs::utils::ReadJsonTreeSteps, utils::{retry_async, read_json_tree}};

use self::types::{Shopify, ShopifyAPIError, Product};


impl Shopify {

    pub fn new (
        shop: &str,
        api_key: &str,
        shared_secret: Option<&str>,
    ) -> Shopify {
        let shop_domain = {
            let mut shop_domain = shop.to_string();
            if !shop_domain.ends_with(".myshopify.com") {
                shop_domain.push_str(".myshopify.com");
            }
            shop_domain
        };

        let query_url = format!(
            "https://{}/admin/api/2023-01/graphql.json",
            shop_domain,

        );
        let rest_url = format!(
            "https://{}/admin/api/2023-01/",
            shop_domain,
        );

        Shopify {
            shared_secret: shared_secret.map(|secret| secret.to_string()),
            api_key: api_key.to_string(),
            query_url,
            rest_url,
            shop: shop.to_string(),
        }
    }

    pub fn get_shop(&self) -> &str {
        self.shop.as_ref()
    }

    pub fn set_api_key(&mut self, api_key: &str) -> Result<&mut Shopify, String> {
        if api_key.is_empty() {
            return Err("API key cannot be empty".to_string());
        }

        self.api_key = api_key.to_string();
        Ok(self)
    }

    /// Get the query url
    pub fn get_query_url(&self) -> &str {
        self.query_url.as_ref()
    }

    /// Get the rest url
    pub fn rest_url(&self) -> &str {
        self.rest_url.as_ref()
    }

    pub fn get_api_endpoint(&self, endpoint: &str) -> String {
        format!("{}{}", self.rest_url(), endpoint)
    }



    pub async fn graphql_query<ReturnType, VariablesType>(
        &self,
        graphql_query: &str,
        variables: &VariablesType,
        json_finder: &Vec<ReadJsonTreeSteps<'_>>,
    ) -> Result<ReturnType, ShopifyAPIError>
    where
        ReturnType: serde::de::DeserializeOwned,
        VariablesType: serde::Serialize,
    {
        let args = (self, graphql_query, variables, json_finder);
        let response_json = retry_async(
            10,
            shopify_graphql_query::<VariablesType, ReturnType>,
            &args,
        )
        .await?;

        Ok(response_json)
    }

pub async fn sync_products(&self) -> Result<Vec<serde_json::Value>, ShopifyAPIError> {
    let query = "query {
        products {
            edges {
                node {
                    id
                    title
                    handle
                }
            }
        }
    }";

    let variables = &();
    let json_finder = vec![
        ReadJsonTreeSteps::Index(1),
        ReadJsonTreeSteps::Key("products"),
        ReadJsonTreeSteps::Key("edges"),
    ];

    let response_json = self
        .graphql_query(query, variables, &json_finder)
        .await?;

    Ok(response_json)
}

}

async fn shopify_graphql_query<VariablesType, ReturnType>(
    (shopify, graphql_query, variables, json_finder): &(
        &Shopify,
        &str,
        &VariablesType,
        &Vec<ReadJsonTreeSteps<'_>>,
    ),
) -> Result<ReturnType, ShopifyAPIError>
where
    VariablesType: serde::Serialize,
    ReturnType: serde::de::DeserializeOwned,
{
    // Prepare the client
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Shopify-Access-Token", shopify.api_key.parse().unwrap());
    let body: &serde_json::Value = &serde_json::json!({
        "query": graphql_query,
        "variables": variables
    });

    // Connection Response
    let res: Response = match client
        .post(shopify.get_query_url())
        .headers(headers)
        .body(body.to_string())
        .send()
        .await
    {
        Ok(local_response) => local_response,
        Err(_) => {
            return Err(ShopifyAPIError::ConnectionFailed);
        }
    };

    // Connection data
    let body = res.text().await;
    if body.is_err() {
        return Err(ShopifyAPIError::ResponseBroken);
    }
    let body = body.unwrap();

    let json: serde_json::Value = {
        match serde_json::from_str(&body) {
            Ok(v) => v,
            Err(_) => {
                // The shopify response is not valid json
                return Err(ShopifyAPIError::NotJson);
            }
        }
    };

    // Check if the query was THROTTLED
    if let Some(error) = json["errors"]["01"]["extensions"]["code"].as_str() {
        if error == "THROTTLED" {
            return Err(ShopifyAPIError::Throttled);
        }
    }

    let json = match read_json_tree(&json, json_finder) {
        Ok(v) => v,
        Err(_) => {
            return Err(ShopifyAPIError::NotWantedJsonFormat);
        }
    };

    let end_json: ReturnType = match serde_json::from_value(json.to_owned()) {
        Ok(v) => v,
        Err(_) => {
            // The shopify response is not wanted json
            return Err(ShopifyAPIError::NotWantedJsonFormat);
        }
    };

    Ok(end_json)
}