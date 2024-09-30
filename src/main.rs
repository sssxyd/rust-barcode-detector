mod basic;
mod service;

use std::{env, io};
use std::io::Write;
use std::path::Path;



fn main() {
    println!("start to detect and decode barcodes");

    // 获取所有命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了至少一个参数
    if args.len() < 1{
        panic!("Please specify the path to save the image");
    }
    let img_path_str = &args[1];
    let pp = Path::new(img_path_str.as_str());
    let mut img_path = pp.to_path_buf();
    if pp.is_relative() {
        let crt = env::current_dir().unwrap();
        img_path = crt.join(pp);
    }
    println!("Image path: {:?}", img_path);
    io::stdout().flush().unwrap(); // 手动刷新

    let gray_image = service::image::read_gray_mat_from_path(img_path.to_str().unwrap()).unwrap();
    let result = service::barcode::detect_and_decode(&gray_image);
    if result.is_err() {
        println!("Failed to detect and decode barcodes: {:?}", result.err().unwrap());
        return;
    }
    let codes = result.unwrap();
    for code in codes {
        println!("Detected barcode: {:?}", code);
    }
}