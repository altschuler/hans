extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;
extern crate iron;
#[macro_use(router)]
extern crate router;

mod routes;
mod core;
mod utils;
mod middleware;

use iron::prelude::*;

use core::error::ErrorHandler;
use utils::rsm::RouteSpecificMiddleware;
use middleware::auth::AuthMiddleware;

fn main() {
    println!("Running...");

    let router = router!(
        // track
        post "/track/add" => RouteSpecificMiddleware::new(routes::track::add, vec![AuthMiddleware], vec![ErrorHandler]),

        // auth
        post "/auth/login" => RouteSpecificMiddleware::new(routes::auth::login, vec![AuthMiddleware], vec![ErrorHandler]));

    Iron::new(router).http("localhost:3000").unwrap();
}
