mod common;

fn main() {
    println!("Hello, world!");
    
    let creditCard = common::model::CreditCard::CreditCard::new("Somchai Prasert".as_ref(), "4242424242424242".as_ref(), 123, 1, 2020);
    println!("{}", creditCard.cardNumber);
    let currency = "thb";
    let res = common::application::PaymentService::charge(&creditCard, 10000, &currency.to_string());
    println!("{:#?}", res);
}
