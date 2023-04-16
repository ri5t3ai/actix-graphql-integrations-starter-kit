use crate::structs::products::CustomProduct;


mod product;
mod order;


fn _transformer_control(payload: &str) -> Result<CustomProduct, serde_json::Error> {
    let json_payload: serde_json::Value = serde_json::from_str(payload)?;
    let id = json_payload["id"].as_u64().unwrap() as u32;
    let title = json_payload["title"].as_str().unwrap();
    let variant_title = json_payload["variant_title"].as_str().unwrap();
    let price = json_payload["price"].as_f64().unwrap() as f32;
    let _handle = match json_payload["handle"].as_str() {
        Some(handle) => handle.to_string(),
        None => {
            let reference = json_payload["reference"].as_str().unwrap();
            let combination_name = json_payload["combination_name"].as_str().unwrap();
            format!("{}-{}", reference, combination_name)
        }
    };

    let platform = match json_payload["handle"].as_str() {
        Some(_) => "Shopify",
        None => "Prestashop"
    };

    Ok(CustomProduct {
        platform: platform.to_string(),
        id: id.to_string(),
        title: title.to_string(),
        description: variant_title.to_string(),
        price,
        quantity: 1,
        type_: "product".to_string(),
    })
}
