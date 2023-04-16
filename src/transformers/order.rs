use crate::{clients::shopify::types::Order  as ShopifyOrder, structs::{utils::ShopSystem, orders::CustomOrder}};



struct OrderTransformer {
}

impl OrderTransformer {
    fn normalize(&self, payload: &str, shop_system: ShopSystem) -> Result<CustomOrder, String> {
        match shop_system {
            ShopSystem::Shopify => {
                let order: ShopifyOrder = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CustomOrder {
         id: "1".to_string(),
                    amount: order.total_price,
                    currency: "USD".to_string(),
                    code: order.order_number.to_string(),


                })
            }
            ShopSystem::PrestaShop => {
                let order: ShopifyOrder = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CustomOrder {
                 id: "1".to_string(),
                    amount: order.total_price,
                    currency: "USD".to_string(),
                    code: order.order_number.to_string(),

                })
            },
                ShopSystem::BigCommerce => {
                let order: ShopifyOrder = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CustomOrder {
                    id: "1".to_string(),
                    amount: order.total_price,
                    currency: "USD".to_string(),
                    code: order.order_number.to_string(),

                })
            }
            ,
                ShopSystem::WooCommerce => {
                let order: ShopifyOrder = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CustomOrder {
                    id: "1".to_string(),
                    amount: order.total_price,
                    currency: "USD".to_string(),
                    code: order.order_number.to_string(),


                })
            }
            ,
                ShopSystem::Shopware => {
                let order: ShopifyOrder = serde_json::from_str(payload).map_err(|e| e.to_string())?;

                Ok(CustomOrder {
                    id: "1".to_string(),
                    amount: order.total_price,
                    currency: "USD".to_string(),
                    code: order.order_number.to_string(),

                })
            }
        }
    }
}
