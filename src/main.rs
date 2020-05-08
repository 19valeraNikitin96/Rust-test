#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rocket_multipart_form_data;
extern crate base64;
extern crate reqwest;

use std::fs;

mod test_api;
mod test_file_helper;
mod api_tests;

fn main() {
    fs::create_dir("images");
    test_api::start();
}
