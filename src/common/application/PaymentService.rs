use std::collections::HashMap;
use crate::common::model;
use crate::common::repository;
use crate::common::domain;

extern crate serde_json;
use serde_json::{Result, Value};

static OMISE_PUBLIC_KEY: &'static str = "pkey_test_5gdksxjz91cfb1w6uov";
static OMISE_SECRET_KEY: &'static str = "skey_test_5fuemctuot0xhqj4xen";
static OMISE_CREATE_TOKEN_ENDPOINT: &'static str = "https://vault.omise.co/tokens";
static OMISE_CHARGE_ENDPOINT: &'static str = "https://api.omise.co/charges";

pub fn charge(creditCard: &model::CreditCard::CreditCard, amount: i32, currency: &String) -> std::string::String {
        
    let tokenString: Option<String> = domain::PaymentDomain::getCardToken(creditCard);
    let tokenString: String = match tokenString{
        Some(token) => token,
        None => "".to_string(),
    };

    let responseCharge = domain::PaymentDomain::chargeWithToken(amount, currency, &tokenString);
    let responseCharge: String = match responseCharge{
        Some(statusString) => statusString,
        None => "failure".to_string(),
    };
    println!("responseCharge {}",responseCharge);

    return responseCharge;
}