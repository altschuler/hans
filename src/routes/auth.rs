extern crate iron;

use iron::prelude::*;
use iron::status;

pub struct AuthToken {
    id: String,
    token: String
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
