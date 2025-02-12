use std::fmt::Display;

pub trait Order {
    fn summarize(&self) -> String;
}

impl<T: Display> Order for T {
    fn summarize(&self) -> String {
        format!("Order Summary: {}", self.to_string())
    }
}
