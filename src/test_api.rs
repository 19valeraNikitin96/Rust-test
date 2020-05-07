use std::vec::Vec;
use rocket::Data;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::ContentType;
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, RawField};
use crate::test_file_helper::{load_2, async_load, image_types, download_thumb_b64_by};
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

#[post("/load/image", data = "<data>")]
fn load(content_type: &ContentType, data: Data) -> JsonValue {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("photo"));
    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let raw_photo = multipart_form_data.raw.get("photo");

    if let Some(raw_photo) = raw_photo {
        match raw_photo {
            RawField::Single(raw) => {
                let content_type = &raw.content_type.as_ref().unwrap().to_string();
                let _file_name = &raw.file_name;
                let _raw = &raw.raw;
                match _file_name {
                    None => {},
                    Some(filename) => {
                        let res = load_2(&content_type, &raw.raw).unwrap();
                        return json!({"status": "ok", "code": 200, "image_id": res});
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
        let pos = item.find(',');
        if pos == None{
            continue;
        }
        let pos = pos.unwrap();
        let t1 = &item[0..pos + 1];
        let b64 = &item[pos + 1..item.len()];
        for t in image_types(){
           match t1.contains(t) {
               true => {
                   let id = load_2(&t.to_owned(), &decode(&b64).unwrap()[..]).unwrap();
                   ids.push(id);
               }
               _ => {}
           }
        }
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

#[get("/download/<image_id>")]
fn download_by(image_id:String) -> JsonValue {
    return match download_thumb_b64_by(image_id) {
        Ok(image_b64) => json!({"status": "ok", "code": 200, "image_b64": image_b64}),
        Err(err) => json!({"status": "failed", "code": 500})
    };
}

pub fn start() {
    rocket::ignite()
        .mount("/", routes![load])
        .mount("/", routes![test_hello])
        .mount("/", routes![load64])
        .mount("/", routes![load_by_url])
        .mount("/", routes![download_by])
        .launch();
}