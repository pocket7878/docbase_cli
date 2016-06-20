extern crate rustc_serialize;

use models::user;
use models::group;
use models::tag;
use models::comment;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub url: String,
    pub created_at: String,
    pub scope: String,
    pub groups: Vec<group::Group>,
    pub tags: Vec<tag::Tag>,
    pub user: user::User,
    pub comments: Vec<comment::Comment>,
}
