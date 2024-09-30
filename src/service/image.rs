use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::prelude::*;
use reqwest::blocking::get;
use std::error::Error;

pub fn read_gray_mat_from_path(image_path: &str) -> Result<Mat, opencv::Error> {
    // 从文件路径读取灰度图片
    let gray_image = imgcodecs::imread(image_path, imgcodecs::IMREAD_GRAYSCALE)?;
    Ok(gray_image)
}

pub fn read_gray_mat_from_base64(base64_str: &str) -> Result<Mat, opencv::Error> {
    // 检测并移除 base64 头部
    let base64_data = if let Some(header_pos) = base64_str.find(",") {
        &base64_str[(header_pos + 1)..]  // 跳过 header
    } else {
        base64_str  // 没有 header，直接处理
    };

    // 使用标准的 Base64 引擎解码
    let decoded_data = STANDARD.decode(base64_data)
        .map_err(|_| opencv::Error::new(0, "Base64 解码失败".to_string()))?;

    // 将解码的数据转换为 OpenCV 的 Vector<u8>
    let image_vector = Vector::<u8>::from(decoded_data);

    // 从内存中的字节数据读取图像，并将其转换为 Mat
    let img = imgcodecs::imdecode(&image_vector, imgcodecs::IMREAD_GRAYSCALE)?;

    // 检查图片是否成功解码
    if img.empty() {
        return Err(opencv::Error::new(0, "无法解码为 Mat 图片".to_string()));
    }

    Ok(img)
}

pub fn read_gray_mat_from_url(url: &str) -> Result<Mat, Box<dyn Error>> {
    // 从 URL 获取图片的二进制数据
    let response = get(url)?.bytes()?;

    // 将 response 的字节数据转为 OpenCV 的 Vector<u8>
    let image_vector = Vector::<u8>::from(response.to_vec());

    // 使用 OpenCV 从内存中的二进制数据解码为 Mat 对象，并转换为灰度图像
    let img = imgcodecs::imdecode(&image_vector, imgcodecs::IMREAD_GRAYSCALE)?;

    // 检查图片是否成功解码
    if img.empty() {
        return Err(Box::from("无法将图片解码为 Mat 对象"));
    }

    Ok(img)
}

