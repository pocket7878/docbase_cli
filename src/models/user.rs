extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub profile_image_url: String
}
