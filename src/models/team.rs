extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Team {
    pub domain: String,
    pub name: String
}
