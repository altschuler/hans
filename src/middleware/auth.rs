extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use iron::BeforeMiddleware;

use routes::data::*;
use errors::*;

pub struct AuthMiddleware;

impl BeforeMiddleware for AuthMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("auth middleware");
        let body = req.get::<bodyparser::Struct<Packet<Empty>>>();
        // let bodyj = req.get::<bodyparser::Json>();

        // match bodyj.unwrap() {
        //     Some(b) => println!("{}", b),
        //     _ => println!("nah")
        // }
        println!("logging in");
        match body {
            Ok(Some(packet)) => authenticate(packet.auth.id),
            _ => Err(IronError::new(NoRoute, status::NotFound))
            //_ => Err(IronError::new(StringError("parsing failed".to_string()), status::BadRequest))
        }
    }
}

pub fn authenticate(id: String) -> IronResult<()> {
    println!("authenticating");
    match id.as_ref() {
        "foo" => Ok(()),
        _ => Err(IronError::new(NoRoute, status::NotFound))
        //_ => Err(IronError::new(StringError("auth failed".to_string()), status::BadRequest))
    }
}
