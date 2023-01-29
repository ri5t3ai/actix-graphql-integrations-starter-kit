#[derive(Clone, Debug)]
pub enum ReadJsonTreeSteps<'a> {
    Key(&'a str),
    Index(usize),
}

#[derive(Clone, Debug)]
pub enum ReadJsonTreeError {
    JsonNotFound,
}


#[derive(Clone, Debug)]
pub enum ShopSystem {
    Shopify,
    PrestaShop,
    BigCommerce,
    WooCommerce,
    Shopware
}
