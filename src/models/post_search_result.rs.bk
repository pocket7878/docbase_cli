extern crate rustc_serialize;

use models::post;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Meta {
    pub previous_page: Option<String>,
    pub next_page: Option<String>,
    pub total: u32
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct PostSearchResult {
    pub posts: Vec<post::Post>,
    pub meta: Meta
}
