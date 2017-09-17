use slack::{self, Event, RtmClient};
use regex::Regex;
use playpen;

pub struct Rustacean;

impl Rustacean {
    fn on_code_present(&self, code: String, channel: String) {
        match playpen::request_eval(&code) {
            Ok(result) => println!("code: {:?}", code),
            Err(err) => println!("error: {:?}", code),
        }
    }
}

#[allow(unused_variables)]
impl slack::EventHandler for Rustacean {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        match event {
            Event::Message(reference) => {
                match *reference {
                    slack::Message::Standard(message) => {
                        println!("message: {:?}", message);

                        match has_code(&message.text){
                            Some(code) =>
                                self.on_code_present(code, message.channel.unwrap()),
                            None =>
                                println!("no code"),
                        }

                    },
                    _ => println!("other")
                }
            },
            _ => println!("other:")
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
        // find the general channel id from the `StartResponse`
        // let general_channel_id = cli.start_response()
        //     .channels
        //     .as_ref()
        //     .and_then(|channels| {
        //                   channels
        //                       .iter()
        //                       .find(|chan| match chan.name {
        //                                 None => false,
        //                                 Some(ref name) => name == "botchannel",
        //                             })
        //               })
        //     .and_then(|chan| chan.id.as_ref())
        //     .expect("general channel not found");
        // let _ = cli.sender().send_message(&general_channel_id, "Hello world! (rtm)");
        // Send a message over the real time api websocket
    }
}

fn has_code(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"```\n?(?P<code>.*?)\n```").unwrap();
            match re.captures(&text) {
                Some(capture) => Some(String::from(&capture["code"])),
                _ => None
            }
        },
        _ => None
    }
}

#[test]
fn test_it_match_code_blocks() {
    let text = "
asdad asd as
```
code
```
";

    let result = has_code(&Some(String::from(text)));

    assert_eq!(result, Some(String::from("code")))
}

#[test]
fn test_it_does_not_match_code_blocks() {
    let text = "
asdad asd as
code
";

    let result = has_code(&Some(String::from(text)));

    println!("{:?}", result);

    assert_eq!(result, None)
}
