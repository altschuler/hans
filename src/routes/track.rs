extern crate bodyparser;

// use persistent::Read;
use rustc_serialize::json;
use iron::prelude::*;
use iron::status;

use core::data::*;
use core::error::*;

pub fn add(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<Packet<Tracking>>>();

    match body {
        Ok(Some(packet)) => match packet.data {
            Some(data) => println!("Hej: {:?},{:?}", data.lat, data.lng),
            None => println!("Fuckup: no data")
        },
        Ok(None) => println!("No hay nada"),
        Err(err) => println!("Fuckup: {}", err)
    }

    let t = Tracking {
        id: "Wat".to_string(),
        lat: 309.0,
        lng: 514.9
    };

    let a = Auth {
        id: "Wat".to_string(),
        token: "watwat".to_string()
    };

    let p = Packet::<Tracking> {
        auth: Some(a),
        kind: PacketType::Tracking,
        data: Some(t)
    };

    let res = json::encode(&p).unwrap();

    Ok(Response::with((status::Ok, res)))
}
