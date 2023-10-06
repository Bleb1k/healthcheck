use clap::Parser;
use reqwest::blocking::Client;
use std::{thread, time::Duration};
use url::Url;

#[derive(Parser, Debug)]
struct Args {
    /// Check interval in seconds
    interval: u64,

    /// URL to check
    url: String,
}

fn main() {
    let args = Args::parse();
    let client = Client::new();
    let url = Url::parse(&args.url).expect("URL parsing error");

    loop {
        let res = client.get(url.as_str()).send();

        match res {
            Ok(data) => println!(
                "Checking '{}'. Result: {}",
                url.as_str(),
                format!(
                    "{}({})",
                    if data.status().is_success() {
                        "OK"
                    } else {
                        "ERR"
                    },
                    data.status().as_u16()
                )
            ),
            Err(err) => panic!("Something went terribly wrong!!!\n{:?}", err),
        }

        thread::sleep(Duration::from_secs(args.interval))
    }
}
