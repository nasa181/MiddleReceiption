use std::collections::HashMap;
use crate::common::model;
use crate::common::repository;

extern crate serde_json;
// use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

static OMISE_PUBLIC_KEY: &'static str = "pkey_test_5gdksxjz91cfb1w6uov";
static OMISE_SECRET_KEY: &'static str = "skey_test_5fuemctuot0xhqj4xen";
static OMISE_CREATE_TOKEN_ENDPOINT: &'static str = "https://vault.omise.co/tokens";
static OMISE_CHARGE_ENDPOINT: &'static str = "https://api.omise.co/charges";

pub fn charge(creditCard: &model::CreditCard::CreditCard, amount: i32, currency: &String) -> std::result::Result<std::string::String, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("card[name]".to_string(), creditCard.nameOnCard.to_string());
    map.insert("card[number]".to_string(), creditCard.cardNumber.to_string());
    map.insert("card[expiration_month]".to_string(), creditCard.expMonth.to_string());
    map.insert("card[expiration_year]".to_string(), creditCard.expYear.to_string());
    map.insert("card[security_code]".to_string(), creditCard.cvv.to_string());

    let mut response = repository::HttpRequestRepository::post(OMISE_CREATE_TOKEN_ENDPOINT, &map, Some(OMISE_PUBLIC_KEY.to_string()), Some("".to_string()));
    let resp: &str = match &response{
        Ok(v) => v,
        Err(e) => "",
    };
    println!("{}",resp);
    let resp: Value = match serde_json::from_str(resp){
        Ok(v) => v,
        Err(e) => Value::Null,
    };
    println!("the token is {}", resp["id"]);

    let mut chargeMap = HashMap::new();
    chargeMap.insert("amount".to_string(), amount.to_string());
    chargeMap.insert("currency".to_string(), currency.to_string());
    chargeMap.insert("card".to_string(), resp["id"].as_str().unwrap().to_string() );
    
    let responseCharge = repository::HttpRequestRepository::post(OMISE_CHARGE_ENDPOINT, &chargeMap, Some(OMISE_SECRET_KEY.to_string()), Some("".to_string()));
    let responseCharge: &str = match &responseCharge{
        Ok(v) => v,
        Err(e) => "",
    };
    println!("responseCharge {}",responseCharge);

    return response;
}