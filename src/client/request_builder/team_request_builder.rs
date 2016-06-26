// docbase_io -- docbase.io cli written in Rust.
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
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/groups", self.team);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let groups: Vec<Group> = json::decode(&buffer).unwrap();
        return groups;
    }

    pub fn send(&self) -> PostSearchResult {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client.get(&format!("https://api.docbase.io/teams/{}/posts", self.team))
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
        let mut res =
            client.get(&format!("https://api.docbase.io/teams/{}/posts?q={}", self.team, q))
                .headers(headers)
                .send()
                .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let searchResult: PostSearchResult = json::decode(&buffer).unwrap();
        return searchResult;
    }

    pub fn tags(&self) -> Vec<Tag> {
        let client = client::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/tags", self.team);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let tags: Vec<Tag> = json::decode(&buffer).unwrap();
        return tags;
    }

    pub fn group(&self, group: String) -> GroupRequestBuilder {
        return GroupRequestBuilder::new(self.api_key.to_owned(), self.team.to_owned(), group);
    }
}
