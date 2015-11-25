extern crate iron;
extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;

// use self::persistent::Read;
// use self::rustc_serialize::json;
use iron::prelude::*;
use iron::status;

pub struct Tracking {
    id: String,
    lat: Vec<i8>,
    long: Vec<i8>,
}

pub fn add(req: &mut Request) -> IronResult<Response> {

    Ok(Response::with(status::Ok))
}
