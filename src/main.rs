extern crate iron;
#[macro_use(router)]
extern crate router;

mod routes;
mod utils;

use iron::prelude::*;
use iron::BeforeMiddleware;
// use router::Router;
use utils::rsm::RouteSpecificMiddleware;

struct AuthMiddleware;

impl BeforeMiddleware for AuthMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("IM IN YR MIDDLEWARE, {}", req.url);
        Ok(())
    }
}

fn main() {
    let router = router!(
        post "/track/add" => RouteSpecificMiddleware::new(routes::track::add, vec![AuthMiddleware]),
        post "/auth/login" => routes::auth::login);

    Iron::new(router).http("0.0.0.0:3000").unwrap();

    println!("Running ");
}
