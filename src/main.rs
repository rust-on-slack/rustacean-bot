extern crate slack;
extern crate regex;
extern crate reqwest;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

mod bot;
mod playpen;

use dotenv::dotenv;
use slack::RtmClient;

fn main() {
    dotenv().ok();

    let api_key = std::env::var("SLACK_API_TOKEN")
        .expect("SLACK_API_TOKEN was not found.");

    let mut handler = bot::Rustacean{};
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    println!("{:?}", r);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
