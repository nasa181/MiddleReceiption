extern crate reqwest;

use std::collections::HashMap;
use crate::common::model;

pub fn get(endPoint: &str,
           username: Option<std::string::String>, 
           password: Option<std::string::String>) -> std::result::Result<std::string::String, reqwest::Error> {
    let client = reqwest::Client::new();
    let httpGetClient = match username {
        Some(user) => client.get(endPoint)
                            .basic_auth(user, password),
                            
        None => client.get(endPoint),
    };
    let resp = httpGetClient.send()?.text();
                
    println!("respnse from get{:#?}", resp);
    return resp;
}

pub fn post(endPoint: &str, 
            payload: &HashMap<String, String>, 
            username: Option<std::string::String>, 
            password: Option<std::string::String>) -> std::result::Result<std::string::String, reqwest::Error> {

    let client = reqwest::Client::new();
    let httpPostClient = match username {
        Some(user) => client.post(endPoint)
                            .basic_auth(user, password),
                            
        None => client.post(endPoint),
    };
    let response = httpPostClient.form(payload)
                                 .send()?
                                 .text();

    return response;
}
