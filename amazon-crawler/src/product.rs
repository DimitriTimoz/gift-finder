use std::convert;
use scraper::{Html, Selector, ElementRef};

#[derive(Debug)]
pub enum Plarform {
    Amazon,
}


#[derive(Debug)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub price: f32,
    pub platform: Plarform,
    pub images_url: Vec<String>,
    pub review: Option<f32>,
    pub nb_review: Option<usize>,
}

impl From<ElementRef<'_>> for Product 
{
    fn from(element: ElementRef) -> Self {


        let title_selector = Selector::parse("h2>a>span").unwrap();
        let price = Selector::parse("span[class='a-price-whole']").unwrap();
        let url_selector = Selector::parse("h2>a").unwrap();
        
        let id = element.value().attr("data-asin").unwrap();
        let title = match element.select(&title_selector).next() {
            Some(value) => value.inner_html(),
            None => String::from(""),
        };
        let price: f32 = match element.select(&price).next() {
            Some(v) => {
                v.inner_html().replace(',', ".").parse().unwrap()},
            None => 0.0,
        };
        let product = Product{
            id: id.to_string(),
            title,
            price,
            platform: Plarform::Amazon,
            images_url: vec![],
            review: None,
            nb_review: None,
        };
        println!("{:?}", product);
        product
    }
}
    


