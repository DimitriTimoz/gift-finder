pub mod request_manager;
pub mod product;


use std::{collections::HashMap, fs, io::Write, thread};


use chrono::{DateTime, Utc, Duration};
use scraper::{Html, Selector};
use crate::product::*;
use lazy_static::lazy_static; 

use std::{sync::Mutex, path::Path, io::{self, BufRead}, fs::File};
use rand::Rng;


lazy_static! {
    static ref PRODUCTS: Mutex<Products> = Mutex::new(Products::default());
    static ref TOR_PROCESS: Mutex<Option<std::process::Child>> = Mutex::new(None);
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
    loop{
        if res.status() != 200 {
            println!("Erreur: {}", res.status());
            
            let mut tor_process = TOR_PROCESS.lock().unwrap().take();
            thread::sleep(std::time::Duration::from_secs(5));

            if let Some(mut p) = tor_process.take(){
                p.kill().unwrap();
                tor_process = None;
            }
            tor_process.replace(std::process::Command::new("tor").spawn().unwrap());
        }else{
            break;
        }
    }
    // Document parsing
    let body = res.text().await?;
    let html = Html::parse_document(&body);

    let s_selector = Selector::parse("div[data-component-type='s-search-result']").unwrap();
    let mut products = PRODUCTS.lock().unwrap();
    let mut total=0;
    let mut n_exists=0;
    for element in html.select(&s_selector) {
        if let Some(product) = Product::from(element) {
            if !products.saved_products.contains_key(&product.id) && !products.to_save.contains_key(&product.id) {
                products.to_save.insert(product.id.clone(), product.clone());
            }else {
                n_exists+=1;
            }
        }
        total+=1;

    }
    println!("Percentage added: {}% ", ((total as f32-n_exists as f32)/(total as f32 + f32::EPSILON))*100.0);
    println!("Total: {}", products.saved_products.len());
    Ok(())
}
    
        


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn save_products() {
    let mut products = PRODUCTS.lock().unwrap();
    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open("products.csv").unwrap();
    for (id, product) in &products.to_save.clone() {
        let record = product.to_csv_line();
        file.write_all(record.as_bytes()).unwrap();
        products.saved_products.insert(id.clone(), SavedProduct::from_product(product));
    }   
    products.to_save.clear();
}

fn read_products(file_name: &str) {
    // TODO: Read products from file
    let file = read_lines(file_name).unwrap();
    for line in file{
        let line = line.unwrap();
        let v: Vec<&str> = line.split(';').collect();
        let id = v[0];
        let title = v[1];
        let price = v[2];
        let _ = v[3];
        let images_url = v[4];
        let review = v[5];
        let nb_review = v[6];
        
        let mut products = PRODUCTS.lock().unwrap();

       // products.insert(product.id.clone(), product.clone());
    }
}

fn read_saved_products(file_name: &str) {
    let file = read_lines(file_name).unwrap();
    for line in file{
        let line = line.unwrap();
        let v: Vec<&str> = line.split(';').collect();
        let id = v[0];
        let platform = v[3];
        let update_date = v[7];
        let product = SavedProduct {
            id: id.to_string(),
            plarform: Plarform::from_string(platform).unwrap(),
            last_update:  DateTime::parse_from_rfc3339(update_date).unwrap().with_timezone(&Utc),
        };
        let mut products = PRODUCTS.lock().unwrap();
        products.saved_products.insert(id.to_string(), product);
    }
}

#[tokio::main]
async fn main() {
    read_saved_products("products.csv");
    let mut tor_process = TOR_PROCESS.lock().unwrap();
    tor_process.replace(std::process::Command::new("tor").spawn().unwrap());
    
    if let Ok(lines) = read_lines("top.csv") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {

            for i in  1..7{
                match get_amazon_product_list(&format!("https://www.amazon.fr/s?k={}&page={}",line, i)).await{
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            }
            save_products();
        }
    }
    let mut tor_process = TOR_PROCESS.lock().unwrap().take();
       
    if let Some(mut p) = tor_process.take(){
        p.kill().unwrap();
    }}
