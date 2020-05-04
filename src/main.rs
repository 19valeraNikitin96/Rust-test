#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rocket_multipart_form_data;
extern crate base64;

mod test_api;
mod test_file_helper;

fn main() {
    test_api::start();
}
