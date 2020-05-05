use std::vec::Vec;

use rocket::Data;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::ContentType;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, RawField};
use crate::test_file_helper::{load_2, async_load};
use base64::decode;

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String
}

#[derive(Serialize, Deserialize)]
struct Base64JSON {
    base64: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct ImageURLsJSON {
    urls: Vec<String>
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
                        let res = load_2(&raw.raw);
                        return json!({"status": "ok", "code": 200, "image_id": res.unwrap()});
                    }
                };
            }
            _ => return json!({"status": "failed", "code": 400})
        };
    }
    json!({"status": "failed", "code": 400})
}

#[post("/load/image64", format = "json", data = "<base64json>")]
fn load64(base64json:Json<Base64JSON>) -> JsonValue {
    let mut ids = Vec::new();
    for item in base64json.0.base64{
        let id = load_2(&decode(item).unwrap()[..]).unwrap();
        ids.push(id);
    }
    json!({"status": "ok", "code": 200, "image_ids": ids})
}

#[post("/load/image-url", format = "json", data = "<json>")]
fn load_by_url(json:Json<ImageURLsJSON>) -> JsonValue {
    let mut ids:Vec<String> = Vec::new();
    for url in json.0.urls{
        let id = async_load(url).unwrap();
        ids.push(id);
    }
    json!({"status": "ok", "code": 200, "image_ids": ids})
}

pub fn start() {
    rocket::ignite()
        .mount("/", routes![load])
        .mount("/", routes![test_hello])
        .mount("/", routes![load64])
        .mount("/", routes![load_by_url])
        .launch();
}