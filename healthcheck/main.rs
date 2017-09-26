extern crate iron;
extern crate mount;

use std::env;
use std::str::FromStr;
use mount::Mount;
use iron::{Iron, Request, Response, IronResult};
use iron::status;

fn handle_index(req: &mut Request) -> IronResult<Response> {
    println!("Running handle_index handler, URL path: {:?}", req.url.path());
    Ok(Response::with((status::Ok, "ok")))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", handle_index);

    let p = std::env::var("PORT").unwrap_or("3000".to_string());
    let port = FromStr::from_str(&p).unwrap();

    println!("Running healthcheck on: http://localhost:{}", port);
    Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}
