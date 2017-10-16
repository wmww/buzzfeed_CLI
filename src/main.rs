extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
//use std::{thread, time};

fn main() {
    let news_api_key = include!("../news_api_key.txt");
    let headlinesURL = format!("http://newsapi.org/v1/articles?source=buzzfeed&sortBy=top&apiKey={}", news_api_key);
    let mut core = Core::new().expect("something fucked up");
    let client = Client::new(&core.handle());
    let uri = headlinesURL.parse().expect("something fucked up");
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());
        let body: String;
        res.body().for_each(|chunk| {
            println!("{}\n\n", String::from_utf8_lossy(chunk.as_ref()));
            //body.push(&chunk);
            //io::stdout()
            //    .write_all(&chunk)
            //    .map_err (From::from)
            Ok(())
        })
    });
    core.run(work).expect("something fucked up");
}
