
use std::fs;
use uuid::Uuid;
use std::io::{Write, Read};
use image::ImageOutputFormat::Png;

pub fn download_thumb_b64_by(image_id:String) -> Result<String, image::ImageError>{
    let path_to = format!("{}{}{}", "images/", image_id, "-100x100.png");
    let mut im = image::open(path_to);
    match im {
        Ok(mut opened) => {
            let mut buf = Vec::new();
            opened.write_to(&mut buf, Png);
            let b64 = base64::encode(&buf);
            Ok(format!("{}{}", "data:image/png;base64,", b64))
        }
        Err(err) => Err(err)
    }
}

pub fn image_types()->Vec<&'static str>{
    vec!["image/gif", "image/png", "image/svg", "image/jpg", "image/jpeg"]
}

pub fn async_load(url:String) -> Result<String, Box<dyn std::error::Error>>{
    let mut resp = reqwest::blocking::get(&url).expect("Request failed");
    let image_id = Uuid::new_v4().to_string();
    let mut out = fs::File::create(format!("{}{}", "images/", image_id))?;
    std::io::copy(&mut resp, &mut out).expect("Failed to copy content");
    Ok(image_id)
}

pub fn load_2(file_type:&str, bytes:&[u8]) -> std::io::Result<String>{
    let end =
        match file_type{
        "image/gif" => ".gif",
        "image/png" => ".png",
        "image/jpg" => ".jpg",
        "image/jpeg" => ".jpeg",
            _ => ""
        };
    write(end, bytes)
}

fn write(end:&str, bytes:&[u8]) -> std::io::Result<String>{
    fs::create_dir("images");
    let image_id = Uuid::new_v4().to_string();
    let path_im = format!("{}{}{}", "images/", image_id, end);
    let mut file = fs::File::create(&path_im).unwrap();
    file.write_all(bytes);
    file.flush();
    let path_thumb = format!("{}{}{}", "images/", image_id, "-100x100.png");
    create_thumb(&path_im, &path_thumb);
    Ok(image_id)
}

fn create_thumb(src_path:&String, dest_path:&String) -> std::io::Result<()>{
    let src = image::open(src_path).unwrap();
    let thumb = src.thumbnail(100, 100);
    thumb.save_with_format(dest_path, image::ImageFormat::Png);
    Ok(())
}