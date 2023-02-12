use common::*;
use request_manager::RequestManager;
use crate::product::*;

use std::thread;
use chrono::{DateTime, Utc};
use scraper::{Html, Selector};
use lazy_static::lazy_static; 
use tokio::sync::Mutex;

pub(crate) mod common;
pub mod request_manager;
pub mod product;
pub mod comments;

lazy_static! {
    static ref PRODUCTS: Mutex<Products> = Mutex::new(Products::default());
    static ref TOR_PROCESS: Mutex<Option<std::process::Child>> = Mutex::new(None);
    static ref REQUEST_MANAGER: Mutex<RequestManager> = Mutex::new(RequestManager::new().unwrap());
}

async fn get_amazon_product_list(url: &str) -> Result<(), reqwest::Error> {
    // Request      
    let body = REQUEST_MANAGER.lock().await.get(url).await?;
    
    // Document parsing
    let html = Html::parse_document(&body);

    let s_selector = Selector::parse("div[data-component-type='s-search-result']").unwrap();
    let mut products = PRODUCTS.lock().await;
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
    

async fn read_products(file_name: &str) {
    // TODO: Read products from file
    let file =  common::read_lines(file_name).unwrap();
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
        
        let mut products = PRODUCTS.lock().await;

       // products.insert(product.id.clone(), product.clone());
    }
}

async fn read_saved_products(file_name: &str) {
    let file = read_lines(file_name).unwrap();
    let mut products = PRODUCTS.lock().await;
    for line in file.skip(1){
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
        products.saved_products.insert(id.to_string(), product);
    }
}

#[tokio::main]
async fn main() {
    
    read_saved_products("products.csv").await;
    
    let mut products = PRODUCTS.lock().await;
    products.load_key_words("top.csv");
    let key_words = products.key_words.clone();
    let mut tor_process = TOR_PROCESS.lock().await;
    tor_process.replace(std::process::Command::new("tor").spawn().unwrap());
    
    drop(tor_process);
    drop(products);

    // Consumes the iterator, returns an (Optional) String
    for (key_word, searched) in key_words {
        if searched {
            continue;
        }
        for i in  1..7{
            match get_amazon_product_list(&format!("https://www.amazon.fr/s?k={}&page={}",key_word, i)).await{
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }
        }
        let mut products = PRODUCTS.lock().await;
        *products.key_words.get_mut(&key_word).unwrap() = true;
        products.save_key_words("top.csv");
        products.save_products();
    }
    
    let mut tor_process = TOR_PROCESS.lock().await.take();
       
    if let Some(mut p) = tor_process.take(){
        p.kill().unwrap();
    }}
