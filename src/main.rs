mod common;

fn main() {
    println!("Hello, world!");
    let url = String::from("https://www.rust-lang.org/learn/get-started");
    let r = common::repository::HttpRequestRepository::get(&url);
    println!("{}", url);

    let creditCard = common::model::CreditCard::CreditCard::new("Somchai Prasert".as_ref(), "4242424242424242".as_ref(), 123, 1, 2020);
    println!("{}", creditCard.cardNumber);
    let res = common::repository::HttpRequestRepository::post(&creditCard);
    println!("{:#?}", res);
}
