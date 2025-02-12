use crate::order::Order;

pub struct Service {
    pub name: String,
    pub duration: u32, // Duration in minutes
}

impl Order for Service {
    fn summarize(&self) -> String {
        format!(
            "Service Summary: Name: {}, Duration: {} minutes",
            self.name, self.duration
        )
    }
}
