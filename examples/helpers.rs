//! This example uses helper functions instead of direct interface.
//!
//! It cannot differentiate between a single and repeated parameter and will
//! fail with 400 as soon as either the body or the required parameter are not
//! provided.

extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use urlencoded::helpers::{require_body_params, require_parameter};

fn log_post_data(req: &mut Request) -> IronResult<Response> {
    let hashmap = try!(require_body_params(req));
    let name = try!(require_parameter(&hashmap, "name"));

    Ok(Response::with((status::Ok, format!("Hello {}", name))))
}

// Test with `curl -i -X POST "http://localhost:3000/" --data "name=world"`
fn main() {
    Iron::new(log_post_data).http("127.0.0.1:3000").unwrap();
}
