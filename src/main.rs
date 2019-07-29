#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket_contrib::json::Json;

mod common;

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[post("/charge", format = "json", data = "<creditCard>")]
fn charge(creditCard: Json<common::model::CreditCard::CreditCard>) -> String {
    format!(
        "nameOnCard: {},
        cardNumber: {},
        cvv: {},
        expMonth: {},
        expYear: {}", 
        creditCard.nameOnCard, creditCard.cardNumber, creditCard.cvv, creditCard.expMonth, creditCard.expYear )
}

fn main() {
    rocket::ignite().mount("/", routes![hello, charge]).launch();
    
    let creditCard = common::model::CreditCard::CreditCard::new("Somchai Prasert".as_ref(), "4242424242424242".as_ref(), 123, 1, 2020);
    println!("{}", creditCard.cardNumber);
    let currency = "thb";
    let res = common::application::PaymentService::charge(&creditCard, 10000, &currency.to_string());
    println!("{:#?}", res);
}
