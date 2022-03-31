
pub mod product;



use scraper::{Html, Selector};
use crate::product::*;
use lazy_static::lazy_static; use core::time;
// 1.4.0
use std::{sync::Mutex, path::Path, io::{self, BufRead}, fs::File, thread};


lazy_static! {
    static ref PRODUCTS: Mutex<Vec<Product>> = Mutex::new(vec![]);
}



async fn get_amazon_product_list(url: &str) -> Result<(), reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
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

    let s_selector = Selector::parse("div[data-asin]").unwrap();
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
            total+=1;
        }
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
    if let Ok(lines) = read_lines("top.json") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {

            for i in  1..7{
                let _re = get_amazon_product_list(&format!("https://www.amazon.fr/s?k={}&page={}",line, i)).await;
                let products = PRODUCTS.lock().unwrap();
                thread::sleep(time::Duration::from_millis(1000));
                println!("{} produits", products.len());
                

            }
            let products = PRODUCTS.lock().unwrap();

            let _ = serde_json::to_writer(&File::create("data.json").unwrap(), &products.as_slice());
                
            
        }
    }
   
    

    
}
