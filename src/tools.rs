use std::collections::HashMap;
use regex::Regex;
use reqwest::Client;
use serde_json::{Value};


#[tokio::main]
pub async fn get_price(_price:&str) -> String {
    // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let response = reqwest::Client::new()
        .get("https://min-api.cryptocompare.com/data/price")
        .query(&[("fsym", _price), ("tsyms", "USD")])
        .header("accept", "application/json")
        .send()
        .await
        .ok().expect("REASON");// Uses ? operator for error propagation
    
    // let prices: &Value = &response.json().await.ok().expect("REASON");
    let prices: String = match response.json::<HashMap<String,f64>>().await
        {
            Ok(num) => {format!("The price of {} is ${}",_price,num["USD"].to_string())},
            Err(_) => {format!("The price of {} is not supported yet",_price) }
    
    
    };
    println!("ETH prices: {:?}", prices);

    return prices;

    // let value:String = format!("{:?}",prices);
    // We know that the price of a cryptocurrency can never be negative
    // We use it as a check if it is a float
    // if prices.get("USD").is_some() {
    //     // let value:String = format!("{:?}",prices["USD"].into());
    //     let value:String = format!("{:?}",prices["USD"]);

    //     return value;
    // } else{

    //     return "Error Fetching Price".to_string();
    // }

}


#[tokio::main]
pub async fn get_marketcap(coin:&str) -> String {

    let token = format!("https://min-api.cryptocompare.com/data/pricemultifull?fsyms={}&tsyms=USD",coin);
        
    let response = reqwest::get(&token).await.expect("REASON");
    
    
    let market_cap : Value =  response.json().await.ok().expect("REASON");
    
    // let prices: HashMap<String, f64> = response.json().await.ok().expect("REASON")
    // println!("ETH prices: {:?}", prices);

    if market_cap.get("DISPLAY").is_some() {
        let json_response = format!("{}",&market_cap.get("DISPLAY")
        .expect("REASON")
        .get(&coin)
        .expect("REASON")
        .get("USD")
        .expect("REASON")
        .get("MKTCAP")
        .unwrap()
        .as_str()
        .unwrap());

        return json_response;
    }else{

        return "Error fetching data!".to_string();
    }

}



pub fn parse_input(input: &str) -> Option<(String, String)> {
    // Trim the brackets
    let trimmed = input.trim().trim_start_matches('[').trim_end_matches(']');

    // Split by comma
    let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();

    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

pub fn extract_output(content: &str) -> Option<String> {
    let output_re = Regex::new(r"=>\. OUTPUT:\s*(.+)(?:\n|$)").unwrap();
    output_re.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}
