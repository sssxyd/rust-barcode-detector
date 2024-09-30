use opencv::core::{add_weighted, convert_scale_abs, DecompTypes, Mat, MatTraitConst, Point2f, Scalar, Size, Vector, BORDER_DEFAULT, CV_16S};
use opencv::imgproc::{adaptive_threshold, equalize_hist, gaussian_blur, get_perspective_transform, sobel, threshold, warp_perspective, ADAPTIVE_THRESH_GAUSSIAN_C, INTER_LINEAR, THRESH_BINARY, THRESH_OTSU};
use opencv::objdetect::{BarcodeDetector, GraphicalCodeDetectorTraitConst};
use crate::basic::Exception;
use crate::service::dto::{CodeInfo, Point};

fn extract_and_rotate_if_needed(image: &Mat, points: &Vec<Point2f>) -> opencv::Result<Mat> {
    let mut src_points = Vector::<Point2f>::new();
    for point in points {
        src_points.push(*point);
    }

    // 计算输出的目标矩形宽高
    let width_a = ((points[1].x - points[0].x).powi(2) + (points[1].y - points[0].y).powi(2)).sqrt();
    let width_b = ((points[2].x - points[3].x).powi(2) + (points[2].y - points[3].y).powi(2)).sqrt();
    let max_width = width_a.max(width_b) as i32;

    let height_a = ((points[0].x - points[3].x).powi(2) + (points[0].y - points[3].y).powi(2)).sqrt();
    let height_b = ((points[1].x - points[2].x).powi(2) + (points[1].y - points[2].y).powi(2)).sqrt();
    let max_height = height_a.max(height_b) as i32;

    // 目标矩形的四个顶点，保持宽高比
    let mut dst_points = Vector::<Point2f>::new();
    dst_points.push(Point2f::new(0.0, 0.0));
    dst_points.push(Point2f::new(max_width as f32, 0.0));
    dst_points.push(Point2f::new(max_width as f32, max_height as f32));
    dst_points.push(Point2f::new(0.0, max_height as f32));

    // 计算透视变换矩阵
    let perspective_matrix = get_perspective_transform(&src_points, &dst_points, i32::from(DecompTypes::DECOMP_LU))?;

    // 透视变换后的图像
    let mut output = Mat::default();
    warp_perspective(
        image,
        &mut output,
        &perspective_matrix,
        Size::new(max_width, max_height),
        INTER_LINEAR,
        0,
        Scalar::default(),
    )?;

    // 如果裁切后的区域是长方形并且高度大于宽度，则旋转90度
    if max_height > max_width {
        let mut rotated = Mat::default();
        opencv::core::transpose(&output, &mut rotated)?; // 转置操作相当于旋转90度
        opencv::core::flip(&rotated, &mut output, 1)?;  // 沿垂直轴翻转
    }

    Ok(output)
}

fn enhance_gray_mat(gray_image: &Mat) -> opencv::Result<Mat> {
    let mut enhanced_image = Mat::default();

    // 1. 使用直方图均衡化来增强对比度
    equalize_hist(gray_image, &mut enhanced_image)?;

    // 2. 使用自适应阈值进一步增强对比度
    let mut temp_image = Mat::default();
    adaptive_threshold(
        &enhanced_image,
        &mut temp_image,
        255.0,
        ADAPTIVE_THRESH_GAUSSIAN_C,
        THRESH_BINARY,
        11,
        2.0,
    )?;
    temp_image.copy_to(&mut enhanced_image)?;

    // 3. 使用高斯模糊去噪
    gaussian_blur(
        &enhanced_image,
        &mut temp_image,
        Size::new(3, 3),
        0.0,
        0.0,
        BORDER_DEFAULT,
    )?;
    temp_image.copy_to(&mut enhanced_image)?;

    // 4. 使用Sobel算子增强边缘细节
    let mut sobel_x = Mat::default();
    let mut sobel_y = Mat::default();
    let mut sobel_combined = Mat::default();

    sobel(&enhanced_image, &mut sobel_x, CV_16S, 1, 0, 3, 1.0, 0.0, BORDER_DEFAULT)?;
    convert_scale_abs(&sobel_x, &mut temp_image, 1.0, 0.0)?;
    temp_image.copy_to(&mut sobel_x)?;

    sobel(&enhanced_image, &mut sobel_y, CV_16S, 0, 1, 3, 1.0, 0.0, BORDER_DEFAULT)?;
    convert_scale_abs(&sobel_y, &mut temp_image, 1.0, 0.0)?;
    temp_image.copy_to(&mut sobel_y)?;

    add_weighted(&sobel_x, 0.5, &sobel_y, 0.5, 0.0, &mut sobel_combined, -1)?;

    // 5. 将Sobel的结果和原图加权合成
    add_weighted(&enhanced_image, 0.8, &sobel_combined, 0.2, 0.0, &mut temp_image, -1)?;
    temp_image.copy_to(&mut enhanced_image)?;

    // 6. 二值化处理（使用Otsu阈值法增强条形码）
    // 使用Otsu's方法找到最佳全局阈值，自动进行二值化处理
    threshold(
        &enhanced_image,
        &mut temp_image,
        0.0,
        255.0,
        THRESH_BINARY | THRESH_OTSU,
    )?;
    temp_image.copy_to(&mut enhanced_image)?;

    Ok(enhanced_image)
}


pub fn detect_and_decode(gray_image: &Mat) -> Result<Vec<CodeInfo>, Exception> {
    let barcode_detector = BarcodeDetector::default().map_err(|e| Exception::new(0, &format!("Failed to create BarcodeDetector: {}", e)))?;
    let mut points = Vector::<Point2f>::new();
    let detect_result = barcode_detector.detect_multi(gray_image, &mut points).map_err(|e| Exception::new(0, &format!("Failed to detect barcodes: {}", e)))?;
    if !detect_result || points.len() < 4 || points.len()%4 != 0 {
        return Err(Exception::new(0, "No barcode detected"));
    }
    let mut results = Vec::<CodeInfo>::new();
    for i in 0..points.len()/4 {
        let mut info_points = Vec::<Point>::new();
        let mut code_points = Vec::new();
        for j in 0..4 {
            let pp = points.get(i * 4 + j).unwrap();
            code_points.push(Point2f::new(pp.x, pp.y));
            info_points.push(Point{
                x: pp.x,
                y: pp.y,
            });
        }
        let code_image = extract_and_rotate_if_needed(gray_image, &code_points).map_err(|e| Exception::new(0, &format!("Failed to extract barcode: {}", e)))?;

        let enhance_mat = enhance_gray_mat(&code_image).map_err(|e| Exception::new(0, &format!("Failed to enhance barcode: {}", e)))?;
        let enhance_points = Vector::<Point2f>::from_slice(&[
            Point2f::new(0.0, 0.0),
            Point2f::new(enhance_mat.cols() as f32, 0.0),
            Point2f::new(enhance_mat.cols() as f32, enhance_mat.rows() as f32),
            Point2f::new(0.0, enhance_mat.rows() as f32),
        ]);
        let mut straight_code = Mat::default();
        let barcode = barcode_detector.decode(&enhance_mat, &enhance_points, &mut straight_code).map_err(|e| Exception::new(0, &format!("Failed to decode barcode: {}", e)))?;
        results.push(CodeInfo{
            code: String::from_utf8(barcode).unwrap(),
            points: info_points,
            category: String::new(),
        });
    }
    Ok(results)
}