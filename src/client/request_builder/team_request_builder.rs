// docbase_io -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use reqwest;
use reqwest::header::Headers;
use serde_json;
use std::io::Read;
use models::post_search_result::PostSearchResult;
use models::group::Group;
use models::tag::Tag;
use client::request_builder::group_request_builder::GroupRequestBuilder;

header! { (XDocBaseToken, "X-DocbaseToken") => [String] }

pub struct TeamRequestBuilder {
    api_key: String,
    team: String,
}

impl TeamRequestBuilder {
    pub fn new(api_key: String, team: String) -> TeamRequestBuilder {
        return TeamRequestBuilder {
            api_key: api_key,
            team: team,
        };
    }

    pub fn groups(&self) -> Vec<Group> {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/groups", self.team);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let groups: Vec<Group> = serde_json::from_str(&buffer).unwrap();
        return groups;
    }

    pub fn send(&self) -> PostSearchResult {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client
            .get(&format!("https://api.docbase.io/teams/{}/posts", self.team))
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
                "https://api.docbase.io/teams/{}/posts?q={}",
                self.team,
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

    pub fn tags(&self) -> Vec<Tag> {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/tags", self.team);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let tags: Vec<Tag> = serde_json::from_str(&buffer).unwrap();
        return tags;
    }

    pub fn group(&self, group: String) -> GroupRequestBuilder {
        return GroupRequestBuilder::new(self.api_key.to_owned(), self.team.to_owned(), group);
    }
}
