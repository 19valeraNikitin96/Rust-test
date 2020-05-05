
use std::fs;
use std::io::prelude::*;
use uuid::Uuid;

pub fn async_load(url:String) -> Result<String, Box<dyn std::error::Error>>{
    let mut resp = reqwest::blocking::get(&url).expect("Request failed");
    let image_id = Uuid::new_v4().to_string();
    let mut out = fs::File::create(format!("{}{}", "images/", image_id))?;
    std::io::copy(&mut resp, &mut out).expect("Failed to copy content");
    Ok(image_id)
}

pub fn load_2(bytes:&[u8]) -> std::io::Result<String>{
    write(bytes)
}

fn write(bytes:&[u8]) -> std::io::Result<String>{
    fs::create_dir("images");
    let image_id = Uuid::new_v4().to_string();
    let mut file = fs::File::create(format!("{}{}", "images/", image_id))?;
    file.write_all(bytes);
    file.flush();
    Ok(image_id)
}