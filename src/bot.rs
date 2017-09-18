use slack::{self, Event, RtmClient};
use regex::Regex;
use playpen;

pub struct Rustacean;

impl Rustacean {
    fn on_message(&self, cli: &RtmClient, message: slack::Message) {
        match message {
            slack::Message::Standard(message) => {
                if let Some(code) = has_code(&message.text) {
                    if let Some(output) = self.eval_code(code) {
                        let channel_id = message.channel.unwrap();
                        let _ = cli.sender()
                                   .send_message(&channel_id, &output);
                    }
                };

                // if let Some(code) = has_bot_mention(&message.text) {
                //     if let Some(output) = self.eval_code(code) {
                //         let channel_id = message.channel.unwrap();
                //         let _ = cli.sender()
                //                    .send_message(&channel_id, &output);
                //     }
                // };
            },
            _ => println!("other")
        }
    }

    fn eval_code(&self, code: String) -> Option<String> {
        match playpen::request_eval(&code) {
            Ok(res) => {
                if !res.success {
                    return None
                }

                Some(format!("output: \n ```\n{}```", res.result()))
            },
            Err(err) => {
                println!("internal error: \n {:?}", err);
                None
            }
        }
    }
}

#[allow(unused_variables)]
impl slack::EventHandler for Rustacean {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        match event {
            Event::Message(reference) => self.on_message(cli, *reference),
            _ => println!("other:")
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

fn has_code(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"```\n?(?s:(?P<code>.*?))\n```").unwrap();
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
    let text = r#"
some message
```
fn main() {
    println!("Hi");
}
```"
"#;

let result = has_code(&Some(String::from(text)));

assert_eq!(result, Some(String::from(r#"fn main() {
    println!("Hi");
}"#)))
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

#[test]
#[ignore]
fn test_request_code_eval() {
    let text = r#"fn main() {
        println!("Hi");
    }

    "#;

    let bot = Rustacean;
    let result = bot.eval_code(String::from(text));

    assert_eq!(result, Some("output: \n ```\nHi\n```".to_string()))
}
