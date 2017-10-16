extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
//use std::{thread, time};

struct HttpThing {
    core: Core,
    client: Client<hyper::client::HttpConnector>,
}

struct HttpResponse {
    status: u32,
    body: String
}

impl HttpThing {

    fn new() -> HttpThing {
        let mut core = Core::new().expect("something fucked up");
        let client = Client::new(&core.handle());
        HttpThing {core, client}
    }

    fn get_blocking(&mut self, uri: &str) -> HttpResponse {
        let uri = uri.parse().expect("uri parsing fucked up");
        let mut status = 0;

        let work = self.client.get(uri).and_then(|res| {
            let mut body = String::new();
            res.body().for_each(|chunk| {
                let cow = String::from_utf8_lossy(chunk.as_ref());
                body.push_str(&cow.into_owned());
                //println!("{}\n\n", String::from_utf8_lossy(chunk.as_ref()));
                //body.push(&chunk);
                //io::stdout()
                //    .write_all(&chunk)
                //    .map_err (From::from)
                Ok(())
            })
        });
        self.core.run(work).expect("something fucked up");
        HttpResponse {status, body}
    }
}

fn main() {
    let news_api_key = include!("../news_api_key.txt");
    let headlinesURL = format!("http://newsapi.org/v1/articles?source=buzzfeed&sortBy=top&apiKey={}", news_api_key);
    let http = HttpThing::new();
    /*let work = client.get(uri).and_then(|res| {
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
    core.run(work).expect("something fucked up");*/
    println!("{}", http.get_blocking(&headlinesURL).body);
}
