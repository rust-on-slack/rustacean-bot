use std::env;

use std::io::Read;
use serde::{Serialize, Deserialize};
use reqwest::{Client, Error};
use reqwest::header::{Headers, ContentType};
use std::str;
use std::collections::HashMap;
use serde_json;

#[derive(Debug, Clone, Serialize)]
pub struct ExecuteRequest {
    channel: String,
    mode: String,
    #[serde(rename = "crateType")]
    crate_type: String,
    tests: bool,
    code: String,
}
impl ExecuteRequest {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            channel: "stable".to_string(),
            crate_type: "bin".to_string(),
            mode: "debug".to_string(),
            tests: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

pub fn request_eval(code: &str) -> Result<ExecuteResponse, Error> {
    let playpen_url = env::var("PLAYPEN_URL").unwrap_or(String::new());

    let code_req = ExecuteRequest::new(code);
    let client = Client::new().unwrap();
    let mut res = client.post(&format!("{}/execute", playpen_url))?
        .json(&code_req)?
        .send()?;

    let content: ExecuteResponse = res.json()?;
    Ok(content)
}

#[test]
fn test_it_executes_code() {
    env::set_var("PLAYPEN_URL", "https://play.rust-lang.org");

    let text = r#"fn main() { println!("hello word"); }"#;

    println!("{:?}", text);
    let result = match request_eval(&text) {
        Ok(res) => res,
        Err(err) => panic!("error: {:?}", err)
    };

    assert_eq!(result.success, true);
    assert_eq!(result.stdout, String::from("hello word\n"))
}

#[test]
fn test_it_executes_ivalid_code() {
    env::set_var("PLAYPEN_URL", "https://play.rust-lang.org");
    let text = r#"
        println!("hello word");
    }
"#;

    let result = match request_eval(&text) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err)
    };

    assert_eq!(result.success, false);
    assert_eq!(result.stderr, String::from(r#"   Compiling playground v0.0.1 (file:///playground)
error: unexpected close delimiter: `}`
 --> src/main.rs:3:5
  |
3 |     }
  |     ^

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
"#))
}


