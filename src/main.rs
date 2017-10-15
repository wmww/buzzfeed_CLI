extern crate hyper;
use hyper::Client;

fn main() {
    let news_api_key = include!("../news_api_key.txt");
    //let client = Client::new();
    println!("{}", news_api_key);
    //let response =
    //client.get("https://newsapi.org/v1/articles?source=buzzfeed&sortBy=top&apiKey={API_KEY}")
    //.send()
    //.expect("Request failed");
    println!("Hello, world!");
}
