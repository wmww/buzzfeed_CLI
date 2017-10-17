extern crate reqwest;

mod wmww_json;

use std::io::Read;
use wmww_json::{parse_json};

fn main() {
    let news_api_key = include!("../news_api_key.txt");
    let headlines_url = format!("http://newsapi.org/v1/articles?source=buzzfeed&sortBy=top&apiKey={}", news_api_key);
    let mut resp = reqwest::get(&headlines_url).expect("making get request fucked up");
    assert!(resp.status().is_success());
    let mut content = String::new();
    resp.read_to_string(&mut content).expect("reading to string fucked up");
    println!("content: {}", content);
    let json = parse_json(content);
}
