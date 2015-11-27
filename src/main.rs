extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;
extern crate iron;
#[macro_use(router)]
extern crate router;

mod routes;
mod utils;
mod middleware;
mod errors;

use iron::prelude::*;
use utils::rsm::RouteSpecificMiddleware;
use middleware::auth::AuthMiddleware;

fn main() {
    let router = router!(
        post "/track/add" => RouteSpecificMiddleware::new(routes::track::add, vec![AuthMiddleware]),
        post "/auth/login" => RouteSpecificMiddleware::new(routes::auth::login, vec![AuthMiddleware]));

    Iron::new(router).http("0.0.0.0:3000").unwrap();

    println!("Running ");
}
