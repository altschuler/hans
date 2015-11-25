extern crate iron;

use iron::{Handler, IronResult, BeforeMiddleware, Request, Response, Chain};

pub struct RouteSpecificMiddleware {
    chain: Chain,
}

impl RouteSpecificMiddleware {
    pub fn new<H: Handler, M: BeforeMiddleware>(handler: H, m: Vec<M>) -> Self {

        let mut chain = Chain::new(handler);

        for item in m.into_iter() {
            chain.link_before(item);
        }

        RouteSpecificMiddleware {
            chain: chain
        }
    }
}

impl Handler for RouteSpecificMiddleware {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.chain.handle(req)
    }
}

// #[macro_export]
// macro_rules! with_middleware {
//     ($handler:ident, before=[$($y:ident),+] ) => {
//         {
//             use selective_middleware::SelectiveMiddleWare;
//             let before = vec!($( $y  ),*);

//             SelectiveMiddleWare::new($handler, before)
//         }
//     };
//     ($handler:ident, [$($y:ident),+] ) => {
//         with_middleware!($handler, before=[$( $y  ),*])
//     }
// }
