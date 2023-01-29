## Create a new Shopify client

 ```
     use shopify_api::*;
     let shopify = Shopify::new("myshop", "myapikey", ShopifyAPIVersion::V2023_01, Some("mysharedsecret"));
     // or without shared secret
     let shopify = Shopify::new("myshop", "myapikey", ShopifyAPIVersion::V2023_01, None);
 ```

## Get the shop name
### Example
```
     use shopify_api::*;
     let shopify = Shopify::new("my-shop", "my-api-key", ShopifyAPIVersion::V2023_01, Some("my-shared-secret"));
     assert_eq!(shopify.get_shop(), "my-shop");
```
## Set the API Key
### Example
 ```
     use shopify_api::*;
     let mut shopify = Shopify::new("myshop", "myapikey", ShopifyAPIVersion::V2023_01, Some("mysharedsecret"));
     shopify.set_api_key("newapikey");
```
### Errors
     This function returns an error if the API key is empty
     
## Get the API endpoint
### Example 



```
     use shopify_api::*;
     let shopify = Shopify::new("myshop", "myapikey", ShopifyAPIVersion::V2023_01, Some("mysharedsecret"));
    
     assert_eq!(shopify.get_api_endpoint("products.json"), "https://myshop.myshopify.com/admin/api/2023-01/products.json");
```

## Query graphql shopify api
### Example

```
     use shopify_api::*;
     use shopify_api::utils::ReadJsonTreeSteps;
     use serde::{Deserialize};
    
     #[derive(Deserialize)]
     struct Shop {
        name: String,
     }
    
     #[tokio::main]
     async fn main() {
       let shopify = Shopify::new(env!("TEST_SHOP_NAME"), env!("TEST_KEY"), ShopifyAPIVersion::V2023_01, None);
       let graphql_query = r#"
          query {
             shop {
              name
             }
         }
       "#;
       let variables = serde_json::json!({});
       let json_finder = vec![ReadJsonTreeSteps::Key("data"), ReadJsonTreeSteps::Key("shop")];
       let shop: Shop = shopify.graphql_query(graphql_query, &variables, &json_finder).await.unwrap();
    
       assert_eq!(shop.name, "Rust api");
     }
```