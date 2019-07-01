extern crate reqwest;

use std::collections::HashMap;
use crate::common::model;

pub fn get(url: &str) -> Result<(), Box<std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get(url)?.json()?;
    println!("{:#?}", resp);
    Ok(())
}

pub fn post(creditCard: &model::CreditCard::CreditCard) -> Result<(), Box<std::error::Error>> {
    let OMISE_PUBLIC_KEY: &str = "pkey_test_5gdksxjz91cfb1w6uov"; //TODO read from config file for separating between test and prod
    let nameOnCard = creditCard.nameOnCard.to_string();
    let cardNumber = creditCard.cardNumber.to_string();
    let expMonth = creditCard.expMonth;
    let expYear = creditCard.expYear;
    let cvv = creditCard.cvv;
    let mut map = HashMap::new();
    map.insert("card[name]", nameOnCard);
    map.insert("card[number]", cardNumber);
    map.insert("card[expiration_month]", expMonth.to_string());
    map.insert("card[expiration_year]", expYear.to_string());
    map.insert("card[security_code]", cvv.to_string());

    let username = "skey_test_5fuemctuot0xhqj4xen";
    let password: Option<&str> = None;

    let client = reqwest::Client::new();
    let mut response = client.post("https://vault.omise.co/tokens")
        .basic_auth(OMISE_PUBLIC_KEY, Some(""))
        .form(&map)
        .send()?
        .text()?;
    println!("{:#?}", response);

    //TODO return to outside function

    Ok(())
}
