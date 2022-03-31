
pub mod product;



use scraper::{Html, Selector};
use crate::product::*;
use lazy_static::lazy_static; 

use std::{sync::Mutex, path::Path, io::{self, BufRead}, fs::File};
use rand::Rng;


lazy_static! {
    static ref PRODUCTS: Mutex<Vec<Product>> = Mutex::new(vec![]);
}



async fn get_amazon_product_list(url: &str) -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
    let client = reqwest::ClientBuilder::new()
            .proxy(proxy)
            .gzip(true)
            .build()?;
    
    // To break the patern
    let mut rng = rand::thread_rng();
    let mut users_agents = read_lines("user-agents.csv").unwrap();
    let n: usize = rng.gen_range(0..1000);
    let users_agents = users_agents.nth(n).unwrap().unwrap();

    // Request
    let res = client.post(url)
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("user-agent", &users_agents)
        .send().await?;

    let body = res.text().await?;
    let html = Html::parse_document(&body);

    let s_selector = Selector::parse("div[data-component-type='s-search-result']").unwrap();
    let mut products = PRODUCTS.lock().unwrap();
    let mut total=0;
    let mut n_exists=0;
    for element in html.select(&s_selector) {
        if let Some(product) = Product::from(element) {
            if !products.iter().any(|p| p.id == product.id) {
                products.push(product);
            }else {
                n_exists+=1;
            }
        }
        total+=1;

    }
    println!("{}/{} ", total-n_exists ,total);
    Ok(())
}
    
        


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() {
    if let Ok(lines) = read_lines("top.csv") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {

            for i in  1..7{
                match get_amazon_product_list(&format!("https://www.amazon.fr/s?k={}&page={}",line, i)).await{
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
                let products = PRODUCTS.lock().unwrap();
                println!("{} produits", products.len());
            }
            let products = PRODUCTS.lock().unwrap();

            let _ = serde_json::to_writer(&File::create("data.json").unwrap(), &products.as_slice());
                
            
        }
    }
   
    

    
}
