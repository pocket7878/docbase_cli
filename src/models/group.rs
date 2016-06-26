// docbase_io -- docbase.io cli written in Rust.
// Copyright 2016 Pocket7878 <poketo7878@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Group {
    pub id: u32,
    pub name: String,
}
