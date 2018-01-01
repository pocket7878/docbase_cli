// docbase_io -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use reqwest;
use reqwest::header::Headers;
use std::io::Read;

use models::team;
use models::post_search_result;
use models::post;
use client::request_builder::team_request_builder::TeamRequestBuilder;
use serde_json;

mod request_builder;

header! { (XDocBaseToken, "X-DocbaseToken") => [String] }

pub struct Client {
    pub api_key: String,
}

impl Client {
    pub fn teams(&self) -> Vec<team::Team> {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let mut res = client
            .get("https://api.docbase.io/teams")
            .headers(headers)
            .send()
            .unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let teams: Vec<team::Team> = serde_json::from_str(&buffer).unwrap();
        return teams;
    }

    pub fn post_detail(&self, domain: &str, post_id: u32) -> post::Post {
        let client = reqwest::Client::new();
        let mut headers = Headers::new();
        headers.set(XDocBaseToken(self.api_key.to_owned()));
        let endpoint_url = format!("https://api.docbase.io/teams/{}/posts/{}", domain, post_id);
        let mut res = client.get(&endpoint_url).headers(headers).send().unwrap();
        let mut buffer = String::new();
        res.read_to_string(&mut buffer).unwrap();
        let post: post::Post = serde_json::from_str(&buffer).unwrap();
        return post;
    }

    pub fn load_prev_post_search_result(
        &self,
        res: &post_search_result::PostSearchResult,
    ) -> post_search_result::PostSearchResult {
        match res.meta.previous_page.to_owned() {
            Some(prev_url) => {
                let client = reqwest::Client::new();
                let mut headers = Headers::new();
                headers.set(XDocBaseToken(self.api_key.to_owned()));
                let mut res = client.get(&prev_url).headers(headers).send().unwrap();
                let mut buffer = String::new();
                res.read_to_string(&mut buffer).unwrap();
                let search_result: post_search_result::PostSearchResult =
                    serde_json::from_str(&buffer).unwrap();
                return search_result;
            }
            None => {
                panic!("No previous url");
            }
        }
    }

    pub fn load_next_post_search_result(
        &self,
        res: &post_search_result::PostSearchResult,
    ) -> post_search_result::PostSearchResult {
        match res.meta.next_page.to_owned() {
            Some(next_url) => {
                let client = reqwest::Client::new();
                let mut headers = Headers::new();
                headers.set(XDocBaseToken(self.api_key.to_owned()));
                let mut res = client.get(&next_url).headers(headers).send().unwrap();
                let mut buffer = String::new();
                res.read_to_string(&mut buffer).unwrap();
                let search_result: post_search_result::PostSearchResult =
                    serde_json::from_str(&buffer).unwrap();
                return search_result;
            }
            None => {
                panic!("No next url");
            }
        }
    }

    pub fn team(&self, team: String) -> TeamRequestBuilder {
        return TeamRequestBuilder::new(self.api_key.to_owned(), team);
    }
}
