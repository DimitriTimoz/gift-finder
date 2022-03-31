use scraper::{Selector, ElementRef};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub enum Plarform {
    Amazon,
}


#[derive(Debug, Serialize, Deserialize)]

pub struct Product {
    pub id: String,
    pub title: String,
    pub price: f32,
    pub platform: Plarform,
    pub images_url: Vec<String>,
    pub review: Option<f32>,
    pub nb_review: Option<usize>,
}

impl Product 
{
    pub fn from(element: ElementRef) -> Option<Self>{

        // List of css selectors
        let title_selector = Selector::parse("h2>a>span").unwrap();
        let price = Selector::parse("span[class='a-price-whole']").unwrap();

        let review_selector = Selector::parse("a>i>span").unwrap();
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
            nb_review: None,
        };
        Some(product)
    }
}
    


