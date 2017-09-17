extern crate slack;
extern crate regex;
extern crate reqwest;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;

mod bot;
mod playpen;

use dotenv::dotenv;
use slack::{Event, RtmClient};

fn main() {
    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    let api_key = std::env::var("SLACK_API_TOKEN")
        .expect("SLACK_API_TOKEN was not found.");

    let mut handler = bot::Rustacean{};
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
