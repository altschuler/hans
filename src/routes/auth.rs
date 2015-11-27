extern crate iron;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;

use routes::data::*;

pub fn login(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<Packet<Empty>>>();
    println!("should create a token");

    Ok(Response::with(status::Ok))

    // let bodyj = req.get::<bodyparser::Json>();

    // match bodyj.unwrap() {
    //     Some(b) => println!("{}", b),
    //     _ => println!("nah")
    // }
    // println!("logging in");
    // match body {
    //     Ok(Some(packet)) => authenticate(packet.auth.id),
    //     _ => Err(IronError::new(StringError("parsing failed".to_string()), status::BadRequest))
    // }
}
