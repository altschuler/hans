extern crate iron;
extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;

use persistent::Read;
use rustc_serialize::json;
use iron::prelude::*;
use iron::status;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Pack  {
    id: String,
    loc: Vec<i8>,
}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let body = req.get::<bodyparser::Raw>();

        match body {
            Ok(Some(b)) => println!("Da bodey is:\n{}", b),
            Ok(None) => println!("No hay nada"),
            Err(err) => println!("Fuckup: {}", err)
        }

        let body = req.get::<bodyparser::Raw>();

        match body {
            Ok(Some(b)) => println!("Da bodey is:\n{}", b),
            Ok(None) => println!("No hay nada"),
            Err(err) => println!("Fuckup: {}", err)
        }

        Ok(Response::with(status::Ok))
    }

    Iron::new(hello_world).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
