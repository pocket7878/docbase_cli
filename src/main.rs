extern crate docbase_cli;

use docbase_cli::client::Client;
use docbase_cli::models::group::Group;
use docbase_cli::models::team::Team;
use docbase_cli::models::post_search_result::PostSearchResult;
use std::env;

fn main() {
    let key = "DOCBASE_TOKEN";
    let api_token = match env::var(key) {
        Ok(v) => v,
        Err(e) => panic!("environment variable `DOCBASE_TOKEN` not found")
    };
    let client = Client { api_key: api_token.to_owned() };
    let searchResult: PostSearchResult = client.posts("xxx");
    if (searchResult.posts.len() > 1) {
        let post = searchResult.posts.first().unwrap();
        println!("{}", post.title);
        println!("{}", post.body);
    }
}
