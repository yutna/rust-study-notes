use ch10_01_traits::aggregator::Summary;
use ch10_01_traits::news_article::NewsArticle;
use ch10_01_traits::notify::notify;
use ch10_01_traits::order::Order;
use ch10_01_traits::product::Product;
use ch10_01_traits::service::Service;
use ch10_01_traits::tweet::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // ---

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // ---
    // Traits as Parameters

    notify(&tweet);
    notify(&article);

    // ---

    let product = Product {
        name: String::from("Laptop"),
        price: 999.99,
    };

    let service = Service {
        name: String::from("Consultation"),
        duration: 60,
    };

    // Call summarize on Product (implements Display)
    println!("{}", product.summarize());

    // Call summarize on Service (does not implement Display)
    println!("{}", service.summarize());
}
