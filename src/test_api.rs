use std::sync::Mutex;
use std::vec::Vec;

use rocket::Data;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::ContentType;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition, FileField, TextField, RawField};
use crate::test_file_helper::load_2;
use base64::{encode, decode};

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String
}

#[derive(Serialize, Deserialize)]
struct Base64JSON {
    name: String,
    base64: String
}

#[get("/hello", format = "json")]
fn test_hello() -> JsonValue {
    json!({"data": "Hello, Valerii!"})
}

//request to "/load/image/load/image"
#[post("/load/image", data = "<data>")]
fn load(content_type: &ContentType, data: Data) -> JsonValue {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("photo"));
    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let raw_photo = multipart_form_data.raw.get("photo");

    if let Some(raw_photo) = raw_photo {
        match raw_photo {
            RawField::Single(raw) => {
                let _content_type = &raw.content_type;
                let _file_name = &raw.file_name;
                let _raw = &raw.raw;
                match _file_name {
                    None => println!("None"),
                    Some(filename) => {
                        load_2(filename.clone(), &raw.raw);
                    }
                };
                // You can now deal with the raw data.
            }
            RawField::Multiple(_bytes) => {
                println!("Some fingerprints");
                // Because we only put one "fingerprint" field to the allowed_fields, this arm will not be matched.
            }
            _ => {}
        };
    }
    json!({"status": "ok", "code": 204})
}

#[post("/load/image64", format = "json", data = "<base64json>")]
fn load64(base64json:Json<Base64JSON>) -> JsonValue {
    load_2(base64json.0.name, &decode(base64json.0.base64).unwrap()[..]);
    json!({"status": "ok", "code": 204})
}


pub fn start() {
    rocket::ignite()
        .mount("/load/image", routes![load])
        .mount("/", routes![test_hello])
        .mount("/load/image64", routes![load64])
        .launch();
}