
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Result;


pub fn load_2(filename: String, bytes:&[u8]) -> std::io::Result<()>{
    write(filename, bytes)
}

fn write(filename:String, bytes:&[u8]) -> std::io::Result<()>{
    fs::create_dir("/images");
    let mut file = fs::File::create(format!("{}{}", "/images/", filename))?;
    println!("-------------------------");
    println!("{:?}", file);
    file.write_all(bytes);
    file.flush();
    Ok(())
}