extern crate rustc_serialize;

use models::user;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Comment {
    pub id: u32,
    pub body: String,
    pub created_at: String,
    pub user: user::User
}
