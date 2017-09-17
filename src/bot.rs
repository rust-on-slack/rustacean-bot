use slack::{self, Event, RtmClient};
use regex::Regex;
use playpen;

pub struct Rustacean;

impl Rustacean {
    fn on_code_present(&self, code: String) -> Option<String> {
        match playpen::request_eval(&code) {
            Ok(res) => {
                match res.result() {
                    Ok(out) => Some(format!("output: \n ```\n{}```", out)),
                    Err(err) => {
                        println!("error: \n ```{}```", &err[50..100]);
                        None
                    },
                }
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
            Event::Message(reference) => {
                match *reference {
                    slack::Message::Standard(message) => {
                        println!("message: {:?}", message);

                        match has_code(&message.text){
                            Some(code) =>{
                                let channel_id = message.channel.unwrap();
                                cli.sender().send_typing(&channel_id);
                                if let Some(output) = self.on_code_present(code) {
                                    let _ = cli.sender()
                                        .send_message(&channel_id,
                                                      &output);
                                }
                            }
                            None => ()
                        };

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
let result = bot.on_code_present(String::from(text));

assert_eq!(result, Some("output: \n ```\nHi\n```".to_string()))
}
