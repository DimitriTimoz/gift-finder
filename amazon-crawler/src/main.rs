
pub mod product;

extern crate select;

use reqwest;

use scraper::{Html, Selector};
use crate::product::*;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;


lazy_static! {
    static ref PRODUCTS: Mutex<Vec<Product>> = Mutex::new(vec![]);
}



async fn get_amazon_product_list(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .build()?;
    let res = client.post(url)
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36")
        .send().await?;

    let body = res.text().await?;

    let html = Html::parse_document(&body);

    let s_selector = Selector::parse("div[data-asin]").unwrap();

    let mut products = PRODUCTS.lock().unwrap();
    for element in html.select(&s_selector) {
        products.push(Product::from(element));
        println!();
    }
    
        
    Ok(String::from(""))
}

#[tokio::main]
async fn main() {
    
    let re = get_amazon_product_list("https://www.amazon.fr/gp/new-releases/?ref_=nav_cs_newreleases").await;
    
    
    
    // Rust call async function synchronously
    // tokio::run(get_amazon_product_list(url));

    
}
