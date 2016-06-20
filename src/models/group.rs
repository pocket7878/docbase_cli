extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Group {
    pub id: u32,
    pub name: String
}
