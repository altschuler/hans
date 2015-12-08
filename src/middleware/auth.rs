extern crate bodyparser;

use iron::prelude::*;
use iron::BeforeMiddleware;

use core::data::*;
use core::error::*;

pub struct AuthMiddleware;

impl BeforeMiddleware for AuthMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // println!("auth middleware");

        let body = req.get::<bodyparser::Struct<Packet<Empty>>>();

        // let bodyj = req.get::<bodyparser::Json>();
        // match bodyj.unwrap() {
        //     Some(b) => println!("{}", b),
        //     _ => println!("nah")
        // }

        match body {
            Ok(Some(packet)) => match packet.auth {
                Some(a) => authenticate(a.id, a.token),
                _ => fail_with(ErrorType::AuthMissing)
            },
            _ => fail_with(ErrorType::ParseFail)
        }
    }
}

pub fn authenticate(id: String, token: String) -> IronResult<()> {
    println!("authenticating id: '{}' token: '{}'", id, token);

    match id.as_ref() {
        "foo" => Ok(()),
        _ => fail_with(ErrorType::AuthMissing),
    }
}
