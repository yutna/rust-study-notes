use crate::aggregator::Summary;

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
