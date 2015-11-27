#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub enum PacketType {
    Tracking,
    TokenRequest
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Empty;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Tracking {
    pub id: String,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Auth {
    pub id: String,
    pub token: String
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Packet<T> {
    pub auth: Auth,
    pub kind: PacketType,
    pub data: Option<T>
}

//{ "auth": { "id": "bar", "token": "wat" }, "data": { "id": "foo", "lat":1.0, "lng":2.0 } }
