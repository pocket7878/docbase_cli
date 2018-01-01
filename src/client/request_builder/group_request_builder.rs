// docbase_io -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use reqwest;
use reqwest::header::Headers;
use std::io::Read;
use serde_json;
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
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client
            .get(&format!(
                "https://api.docbase.io/teams/{}/posts?q=group:\"{}\"",
                self.team,
                self.group
            ))
            .headers(headers)
            .send()
            .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let search_result: PostSearchResult = serde_json::from_str(&buffer).unwrap();
        return search_result;
    }

    pub fn search(&self, q: &str) -> PostSearchResult {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client
            .get(&format!(
                "https://api.docbase.io/teams/{}/posts?q=group:\"{}\" {}",
                self.team,
                self.group,
                q
            ))
            .headers(headers)
            .send()
            .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let search_result: PostSearchResult = serde_json::from_str(&buffer).unwrap();
        return search_result;
    }
}
