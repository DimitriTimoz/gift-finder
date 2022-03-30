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


        let title_selector = Selector::parse("h2>a>span").unwrap();
        let price = Selector::parse("span[class='a-price-whole']").unwrap();

        let review_selector = Selector::parse("a>i>span").unwrap();
        let id = match element.value().attr("data-asin"){
            Some(id) => id.to_string(),
            None => return None,
        };
        
        let title = match element.select(&title_selector).next() {
            Some(value) => value.inner_html(),
            None => return None,
        };
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

        let review = match element.select(&review_selector).next() {
            Some(v) => {
               
                let v4 = v.inner_html().replace(',', ".");
                match v4[0..3].parse::<f32>() {
                    Ok(v) => Some(v/5.0),
                    Err(_) => None,
                }   
                                 
            },
            None => None,
        };
            
        
        let product = Product{
            id,
            title,
            price,
            platform: Plarform::Amazon,
            images_url: vec![],
            review,
            nb_review: None,
        };
        Some(product)
    }
}
    


