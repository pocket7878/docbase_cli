// docbase_io -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
use models::user;
use models::group;
use models::tag;
use models::comment;

#[derive(Serialize, Deserialize)]
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
