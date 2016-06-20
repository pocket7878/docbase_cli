extern crate rustc_serialize;

use hyper::client;
use hyper::header::Headers;
use std::io::Read;
use rustc_serialize::json;

use models::team;
use models::group;
use models::post_search_result;

header! { (XDocBaseToken, "X-DocbaseToken") => [String] }

pub struct Client {
    pub api_key: String
}

impl Client {
    pub fn teams(&self) -> Vec<team::Team> {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client.get("https://api.docbase.io/teams").headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let teams: Vec<team::Team> = json::decode(&buffer).unwrap();
        return teams;
    }

    pub fn groups(&self, domain: &str) -> Vec<group::Group> {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/groups", domain);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let groups: Vec<group::Group> = json::decode(&buffer).unwrap();
        return groups;
    }

    pub fn get_posts(&self, domain: &str) -> post_search_result::PostSearchResult {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/posts", domain);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let searchResult: post_search_result::PostSearchResult = json::decode(&buffer).unwrap();
        return searchResult;
    }
}
