use std::collections::HashMap;
use regex::Regex;
use reqwest::Client;


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
    
    let prices: HashMap<String, f64> = response.json().await.ok().expect("REASON");
    println!("ETH prices: {:?}", prices);

    // let value:String = format!("{:?}",prices);
    let value:String = format!("{}",prices["USD"]);

    return value;

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
