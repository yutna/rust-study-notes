use std::fmt::Display;

pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Product: {}, Price: ${:.2}", self.name, self.price)
    }
}
