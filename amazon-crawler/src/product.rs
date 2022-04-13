use std::{collections::HashMap, fs, io::Write};
use crate::common::*;
use scraper::{Selector, ElementRef};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Default)]
pub struct Products{
    pub saved_products: HashMap<String, SavedProduct>,
    pub to_save: HashMap<String, Product>,
    pub key_words: HashMap<String, bool>,
}

impl Products{
    pub fn save_key_words(&mut self, file_name: &str){
        let mut file = fs::OpenOptions::new()
                                    .write(true)
                                    .open(file_name).unwrap();
        for (key_word, saved) in &self.key_words {
            let line = format!("{};{}\n", key_word, saved);
            file.write_all(line.as_bytes()).unwrap();
        }
    }

    pub fn load_key_words(&mut self, file_name: &str){
        let lines = read_lines(file_name);
        for line in lines.unwrap().flatten(){
            let args = line.split(';').collect::<Vec<&str>>();
            if args.len() > 1{
                self.key_words.insert(args[0].to_string(), args[1].parse::<bool>().unwrap());
            }else{
                // To add keywords without specify if they already have been searched
                self.key_words.insert(args[0].to_string(), false);
            }
        }
    }


pub fn save_products(&mut self) {
    // Save products to csv format
    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open("products.csv").unwrap();
    if self.saved_products.is_empty() {
        file.write_all(Product::header_csv().as_bytes()).unwrap();
    }
    for (id, product) in self.to_save.clone() {
        let record = product.to_csv_line();
        file.write_all(record.as_bytes()).unwrap();
        self.saved_products.insert(id.clone(), SavedProduct::from_product(&product));
    }   
    self.to_save.clear();
}

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Plarform {
    Amazon,
}

impl Plarform {
    #[inline]
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "1" => Some(Plarform::Amazon),
            _ => None,
        }
    }
}

impl ToString for Plarform {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            Plarform::Amazon => "1".to_string(),
        }
    }
}

#[derive(Debug, Clone)]

pub struct Product {
    pub id: String,
    pub title: String,
    pub price: f32,
    pub platform: Plarform,
    pub images_url: Vec<String>,
    pub review: Option<f32>,
    pub nb_review: Option<u32>,
    pub date: DateTime<Utc>,
}

impl Product 
{
    pub fn from(element: ElementRef) -> Option<Self>{

        // List of css selectors
        let title_selector = Selector::parse("h2>a>span").unwrap();
        let price = Selector::parse("span[class='a-price-whole']").unwrap();
        let review_selector = Selector::parse("a>i>span").unwrap();
        let nb_review_selector = Selector::parse("a>span[class='a-size-base s-underline-text']").unwrap();
        let images_selector = Selector::parse("img[class='s-image']").unwrap();
        
        // Select the id of the product
        let id = match element.value().attr("data-asin"){
            Some(id) => id.to_string(),
            None => return None,
        };
        
        // Select the title
        let title = match element.select(&title_selector).next() {
            Some(value) => value.inner_html(),
            None => return None,
        };

        // Select the price
        let price: f32 = match element.select(&price).next() {
            Some(v) => {
                if let Ok(v2) = v.inner_html().replace(',', ".").parse(){
                    v2
                }else{
                    return None;
                }
            },
            None => return None,
        };

        // Select review average
        let review = match element.select(&review_selector).next() {
            Some(v) => {
                let v4 = v.inner_html().replace(',', ".");
                match v4[0..3].parse::<f32>() {
                    Ok(v) => {
                        Some(v/5.0)
                    },
                    Err(_) => None,
                }                
            },
            None => None,
        };
        
        // Select the number of review if exists
        let nb_review = if review.is_some() {
            match element.select(&nb_review_selector).next() {
                Some(v) => {
                    let v1 = v.inner_html();
                    match v1.parse::<u32>() {
                        Ok(v) => {
                            Some(v)
                        },
                        Err(_) => None,
                    }                
                },
                None => None,
            }
        }
        else {None};

        // Select the images url
        let mut images_url = Vec::new();
        for image in element.select(&images_selector) {
            if let Some(url) = image.value().attr("srcset") {
                url.split(',').for_each(|url| {
                    let s = url.replace("https://m.media-amazon.com/images/I/", "").split(' ').next().unwrap_or("").to_string();

                    if s.len() > 1{
                         images_url.push(s[1..].split(' ').next().unwrap().to_string());

                    }
                });
            }
        }
            
        let product = Product{
            id,
            title,
            price,
            platform: Plarform::Amazon,
            images_url,
            review,
            nb_review,
            date: Utc::now(),
        };
        Some(product)
    }

    pub fn to_csv_line(&self) -> String{
        let p = self.review.map(|v| format!("{}", v)).unwrap_or_else(|| "null".to_string()).replace(';', "%3B");
        let v =  self.nb_review.map(|v| format!("{}", v)).unwrap_or_else(|| "null".to_string()).replace(';', "%3B");
        format!("\n{};{};{};{};{};{};{};{}", self.id.replace(';', "%3B"), self.title.replace(';', "%3B"), self.price, self.platform.to_string(), self.images_url.join(",").replace(';', "%3B"), p, v, self.date.to_rfc3339())
    }

    #[inline]
    pub fn header_csv() -> String{
        "id;title;price;platform;images_url;review;nb_review;date".to_string()
    }
}
    
#[derive(Debug, Clone)]
pub struct SavedProduct{
    pub id: String,
    pub last_update: DateTime<Utc>,
    pub plarform: Plarform,
}

impl SavedProduct{
    #[inline]
    pub fn to_csv_line(&self) -> String {
        format!("{},{},{}", self.id, self.last_update.to_rfc3339(), self.plarform.to_string())
    }

    #[inline]
    pub fn from_product(product: &Product) -> Self{
        SavedProduct {
            id: product.id.clone(),
            last_update: Utc::now(),
            plarform: product.platform.clone(),
        }
    }
}