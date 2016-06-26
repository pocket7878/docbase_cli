// docbase_cli -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use hyper::client;
use hyper::header::Headers;
use std::io::Read;
use rustc_serialize::json;
use models::post_search_result::PostSearchResult;

header! { (XDocBaseToken, "X-DocbaseToken") => [String] }

pub struct GroupRequestBuilder {
    api_key: String,
    team: String,
    group: String,
}

impl GroupRequestBuilder {
    pub fn new(api_key: String, team: String, group: String) -> GroupRequestBuilder {
        return GroupRequestBuilder {
            api_key: api_key,
            team: team,
            group: group,
        };
    }

    pub fn send(&self) -> PostSearchResult {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client.get(&format!("https://api.docbase.io/teams/{}/posts?q=group:\"{}\"",
                          self.team,
                          self.group))
            .headers(headers)
            .send()
            .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let searchResult: PostSearchResult = json::decode(&buffer).unwrap();
        return searchResult;
    }

    pub fn search(&self, q: &str) -> PostSearchResult {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client.get(&format!("https://api.docbase.io/teams/{}/posts?q=group:\"{}\" {}",
                          self.team,
                          self.group,
                          q))
            .headers(headers)
            .send()
            .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let searchResult: PostSearchResult = json::decode(&buffer).unwrap();
        return searchResult;
    }
}
