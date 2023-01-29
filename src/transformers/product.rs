use serde_json::Value;

use crate::{ clients::shopify::types::Product as ShopifyProduct, structs::{products::CharlesProduct, utils::ShopSystem}};




struct ProductTransformer {
}

impl ProductTransformer {
       fn normalize(&self, payload: &str, shop_system: ShopSystem) -> Result<CharlesProduct, String> {
        match shop_system {
            ShopSystem::Shopify => {
                let product: ShopifyProduct = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CharlesProduct {
                     id: "1".to_string(),
                     platform: "Shopify".to_string(),
                    title: product.title,
                    description: "Shopify".to_string(),
                    price: 3.0,
                    quantity: product.variants[0].inventory_quantity,
                    type_: product.product_type,
                })
            }
            ShopSystem::PrestaShop => {
                let product: ShopifyProduct = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CharlesProduct {
                     id: "1".to_string(),
                     platform: "Shopify".to_string(),
                    title: product.title,
                    description: "Shopify".to_string(),
                    price: 3.0,
                    quantity: product.variants[0].inventory_quantity,
                    type_: product.product_type,

                })
            },
                ShopSystem::BigCommerce => {
                let product: ShopifyProduct = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CharlesProduct {
                         id: "1".to_string(),
                     platform: "Shopify".to_string(),
                    title: product.title,
                    description: "Shopify".to_string(),
                    price: 3.0,
                    quantity: product.variants[0].inventory_quantity,
                    type_: product.product_type,

                })
            }
            ,
                ShopSystem::WooCommerce => {
                let product: ShopifyProduct = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CharlesProduct {
                        id: "1".to_string(),
                     platform: "Shopify".to_string(),
                    title: product.title,
                    description: "Shopify".to_string(),
                    price: 3.0,
                    quantity: product.variants[0].inventory_quantity,
                    type_: product.product_type,


                })
            }
            ,
                ShopSystem::Shopware => {
                let product: ShopifyProduct = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CharlesProduct {
                id: "1".to_string(),
                     platform: "Shopify".to_string(),
                    title: product.title,
                    description: "Shopify".to_string(),
                    price: 3.0,
                    quantity: product.variants[0].inventory_quantity,
                    type_: product.product_type,

                })
            }
        }
    }

    // define a translate function that takes in the CharlesProcut and returns a platform specific product based on the platform field
    fn _translate(&self, payload: serde_json::Value, shop_system: ShopSystem) -> Result<ShopifyProduct, String> {
//     let id = payload.id.clone();
// let id_i64: i64 = match id.parse() {
//     Ok(n) => n,
//     Err(_) => {
//         // handle the error here, for example by returning a default value
//         return Err("id is not a number".to_string());
//     }

// };

      match shop_system {
            ShopSystem::Shopify => {
                let product: ShopifyProduct = serde_json::from_value(payload).map_err(|e| e.to_string())?;

                Ok(product)
            }
            ShopSystem::PrestaShop => {
                let product: ShopifyProduct = serde_json::from_value(payload).map_err(|e| e.to_string())?;

                Ok(product)
            },
                ShopSystem::BigCommerce => {
                let product: ShopifyProduct = serde_json::from_value(payload).map_err(|e| e.to_string())?;

                Ok(product)
            }
            ,
                ShopSystem::WooCommerce => {
                let product: ShopifyProduct = serde_json::from_value(payload).map_err(|e| e.to_string())?;

                Ok(product)
            }
            ,
                ShopSystem::Shopware => {
                let product: ShopifyProduct = serde_json::from_value(payload).map_err(|e| e.to_string())?;

                Ok(product)
            }
        }

    }
}