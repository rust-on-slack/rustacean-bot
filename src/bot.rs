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
                } else if let Some(command) = has_command(&message.text) {
                    if let Some(output) = self.eval_command(command) {
                        let channel_id = message.channel.unwrap();
                        let _ = cli.sender()
                                   .send_message(&channel_id, &output);
                    }
                } else if let Some(output) = has_bot_mention(&message.text) {
                    let channel_id = message.channel.unwrap();
                    let _ = cli.sender().send_message(&channel_id, &output);
                };
            }
            _ => println!("other"),
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

    fn eval_command(&self, command: String) -> Option<String> {
        match command.to_lowercase().as_str() {
            "docs" => Some("https://doc.rust-lang.org/".to_owned()),
            "book" => Some("https://doc.rust-lang.org/book/second-edition/".to_owned()),
            _ => None
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

fn has_command(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let re = Regex::new(r"/help (?P<command>.*?)$").unwrap();
            match re.captures(&text) {
                Some(capture) => Some(String::from(&capture["command"])),
                _ => None
            }
        },
        _ => None
    }
}

fn has_bot_mention(message: &Option<String>) -> Option<String> {
    match message {
        &Some(ref text) => {
            let bot_name = env::var("SLACK_BOT_NAME").expect("SLACK_BOT_NAME was not found.");
            let re = Regex::new(r"@(?P<bot>[\w_]+)").unwrap();
            for caps in re.captures_iter(&text) {
                if bot_name == &caps["bot"] {
                    return Some(String::from("Hi there!"));
                };
            }
            None
        }
        _ => None,
    }
}


#[test]
fn test_has_bot_mentions() {
    use dotenv::dotenv;
    dotenv().ok();

    let text = r#"Hey @BotName u there?"#;
    let result = has_bot_mention(&Some(String::from(text)));
    assert_eq!(result, Some(String::from("Hi there!")))
}

#[test]
fn test_has_bot_mentions_with_others_mentions() {
    use dotenv::dotenv;
    dotenv().ok();

    let text = r#"Hey @Mario or @BotName are there?"#;
    let result = has_bot_mention(&Some(String::from(text)));
    assert_eq!(result, Some(String::from("Hi there!")))
}

#[test]
fn test_match_command() {
    let text = r#"/help docs"#;

    let result = has_command(&Some(String::from(text)));

    assert_eq!(result, Some(String::from(r#"docs"#)))
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

