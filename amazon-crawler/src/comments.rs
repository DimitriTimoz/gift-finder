

pub struct Review {
    pub author: String,
    pub bought: bool,
    pub title: String,
    pub content: String,
    pub location: String,
    pub date: String,
    pub rating: u8,
    pub helpful: u32,
}

pub struct Rating {
    pub rating: u8,
    pub amount: u32,
}

pub struct Reviews {
    pub reviews: Vec<Review>,
    pub amount: u32,
    pub ratings: [Rating; 5],
}

impl Reviews {
    pub async fn request_reviews() -> Self {




        Self{
            reviews: Vec::new(),
            amount: 0,
            ratings: [Rating{rating: 1, amount: 0}, Rating{rating: 2, amount: 0}, Rating{rating: 3, amount: 0}, Rating{rating: 4, amount: 0}, Rating{rating: 5, amount: 0}],
        }
    }
}

