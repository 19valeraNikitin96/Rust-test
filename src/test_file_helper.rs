
use std::fs;
use uuid::Uuid;
use std::io::{Write, Error, ErrorKind};
use image::ImageOutputFormat::Png;
use std::collections::HashMap;

pub fn download_thumb_b64_by(image_id:String) -> std::io::Result<String>{
    let path_to = format!("{}{}{}", "images/", image_id, "-100x100.png");
    let mut im = image::open(path_to);
    match im {
        Ok(mut opened) => {
            let mut buf = Vec::new();
            opened.write_to(&mut buf, Png);
            let b64 = base64::encode(&buf);
            Ok(format!("{}{}", "data:image/png;base64,", b64))
        }
        Err(err) => Result::Err(Error::new(ErrorKind::Other, "File not found"))
    }
}

pub fn image_formats()->HashMap<&'static str, &'static str>{
    let mut f = HashMap::new();
    f.insert("gif", "image/gif");
    f.insert("png", "image/png");
    f.insert("svg", "image/svg");
    f.insert("jpg", "image/jpg");
    f.insert("jpeg", "image/jpeg");
    f
}

pub fn upload_by_url(url:String) -> std::io::Result<String>{
    let last_dot = url.rfind('.').unwrap();
    let extension = &url[last_dot+1..];
    println!("{}", extension);
    match image_formats().get(extension) {
        Some(data)=> {
            let mut resp = reqwest::blocking::get(&url).expect("Request failed");
            let mut bytes = Vec::new();
            std::io::copy(&mut resp, &mut bytes);
            upload(data, &bytes)
        },
        None => Result::Err(Error::new(ErrorKind::Other, "Undefined type of image"))
    }
}

pub fn upload(file_type:&str, bytes:&[u8]) -> std::io::Result<String>{
        match file_type{
        "image/gif"  => write(".gif", bytes),
        "image/png"  => write(".png", bytes),
        "image/jpg"  => write(".jpg", bytes),
        "image/jpeg" => write(".jpeg", bytes),
        "image/svg"  => write(".svg", bytes),
            _ => Result::Err(Error::new(ErrorKind::Other, "Undefined type of image"))
        }
}

fn write(end:&str, bytes:&[u8]) -> std::io::Result<String>{
    let image_id = Uuid::new_v4().to_string();
    let path_im = format!("{}{}{}", "images/", image_id, end);
    let mut file = fs::File::create(&path_im).unwrap();
    file.write_all(bytes);
    file.flush();
    let path_thumb = format!("{}{}{}", "images/", image_id, "-100x100.png");
    create_thumb(&path_im, &path_thumb);
    Ok(image_id)
}

fn create_thumb(src_path:&String, dest_path:&String){
    let src = image::open(src_path).unwrap();
    let thumb = src.thumbnail(100, 100);
    thumb.save_with_format(dest_path, image::ImageFormat::Png);
}