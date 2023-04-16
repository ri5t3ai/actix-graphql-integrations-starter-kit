use reqwest::Client;
use serde::{Deserialize, Serialize};

const PRESTASHOP_API_URL: &str = "https://{}/api";

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    key: String,
}

#[derive(Debug, Deserialize)]
struct Resource<T> {
    #[serde(rename = "prestashop")]
    prestashop_resource: T,
}

#[derive(Debug, Deserialize)]
struct Shop {
    #[serde(rename = "name")]
    shop_name: String,
}

struct PrestaShop {
    api_key: String,
    shop_name: String,
    client: Client,
}

impl PrestaShop {
    pub fn new(api_key: &str, shop_name: &str) -> Self {
        let client = Client::new();
        Self {
            api_key: api_key.to_owned(),
            shop_name: shop_name.to_owned(),
            client,
        }
    }

    pub async fn authenticate(&self) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = format!(PRESTASHOP_API_URL, &self.shop_name);

        let key = Key { key: self.api_key.clone() };

        let response = self
            .client
            .post(&api_url)
            .json(&key)
            .send()
            .await?
            .json::<Resource<Shop>>()
            .await?;

        Ok(response.prestashop_resource.shop_name)
    }


    pub async fn sync_products(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let api_url = format!("{}/products", PRESTASHOP_API_URL);

        let mut page = 1;
        let mut products = Vec::new();

        loop {
            let response = self
                .client
                .get(&api_url)
                .query(&[("page", page), ("limit", RESOURCES_PER_PAGE)])
                .send()
                .await?
                .json::<Resource<Vec<Product>>>()
                .await?;

            let fetched_products = response.prestashop_resource;
            if fetched_products.is_empty() {
                break;
            }

            for product in fetched_products {
                products.push(product.product_name);
            }

            page += 1;
        }

        Ok(products)
    }

    pub async fn sync_orders(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_url = format!("{}/orders", PRESTASHOP_API_URL);

    let mut page = 1;
    let mut orders = Vec::new();

    loop {
        let response = self
            .client
            .get(&api_url)
            .query(&[("page", page), ("limit", RESOURCES_PER_PAGE)])
            .send()
            .await?
            .json::<Resource<Vec<Order>>>()
            .await?;

        let fetched_orders = response.prestashop_resource;
        if fetched_orders.is_empty() {
            break;
        }

        for order in fetched_orders {
            orders.push(order.order_reference);
        }

        page += 1;
    }

    Ok(orders)
}
pub async fn sync_customers(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_url = format!("{}/customers", PRESTASHOP_API_URL);

    let mut page = 1;
    let mut customers = Vec::new();

    loop {
        let response = self
            .client
            .get(&api_url)
            .query(&[("page", page), ("limit", RESOURCES_PER_PAGE)])
            .send()
            .await?
            .json::<Resource<Vec<Customer>>>()
            .await?;

        let fetched_customers = response.prestashop_resource;
        if fetched_customers.is_empty() {
            break;
        }

        for customer in fetched_customers {
            customers.push(customer.email);
        }

        page += 1;
    }

    Ok(customers)
}
    pub async fn create_cart(&self, cart: Cart) -> Result<Cart, reqwest::Error> {
        let client = Client::new();
        let response = client
            .post(&format!(
                "{}/api/carts?schema=blank",
                self.base_url
            ))
            .basic_auth(self.api_key, Some(""))
            .json(&cart)
            .send()
            .await?
            .json::<Cart>()
            .await?;

        Ok(response)
    }
}