
pub mod product;



use scraper::{Html, Selector};
use crate::product::*;
use lazy_static::lazy_static; // 1.4.0
use std::{sync::Mutex, path::Path, io::{self, BufRead}, fs::File};


lazy_static! {
    static ref PRODUCTS: Mutex<Vec<Product>> = Mutex::new(vec![]);
}



async fn get_amazon_product_list(url: &str) -> Result<(), reqwest::Error> {
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
        if let Some(product) = Product::from(element) {
            products.push(product);
        }
    }
    Ok(())
}
    
        


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() {
    if let Ok(lines) = read_lines("top.json") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {

            for i in  1..7{
                let _re = get_amazon_product_list(&format!("https://www.amazon.fr/s?k={}&page={}",line, i)).await;

            }
            let mut products = PRODUCTS.lock().unwrap();

            serde_json::to_writer(&File::create("data.json").unwrap(), &products.as_slice());
                
            
        }
    }
   
    
    
    
    // Rust call async function synchronously
    // tokio::run(get_amazon_product_list(url));

    
}
