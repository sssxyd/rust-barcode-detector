use opencv::core::{add_weighted, convert_scale_abs, DecompTypes, Mat, MatTraitConst, Point2f, Scalar, Size, Vector, BORDER_CONSTANT, BORDER_DEFAULT, CV_16S, CV_8U};
use opencv::imgcodecs::imwrite;
use opencv::imgproc;
use opencv::imgproc::{adaptive_threshold, equalize_hist, gaussian_blur, laplacian, get_perspective_transform, sobel, threshold, warp_perspective, ADAPTIVE_THRESH_GAUSSIAN_C, INTER_LINEAR, THRESH_BINARY, THRESH_OTSU, morphology_ex, MORPH_CLOSE};
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

fn extract_and_expand(image: &Mat, points: &Vec<Point2f>) -> opencv::Result<Mat> {
    let mut src_points = Vector::<Point2f>::new();
    for point in points {
        src_points.push(*point);
    }

    // 计算输出的目标矩形宽高
    let width_a = ((points[1].x - points[0].x).powi(2) + (points[1].y - points[0].y).powi(2)).sqrt();
    let width_b = ((points[2].x - points[3].x).powi(2) + (points[2].y - points[3].y).powi(2)).sqrt();
    let max_width = width_a.max(width_b) as f32;

    let height_a = ((points[0].x - points[3].x).powi(2) + (points[0].y - points[3].y).powi(2)).sqrt();
    let height_b = ((points[1].x - points[2].x).powi(2) + (points[1].y - points[2].y).powi(2)).sqrt();
    let max_height = height_a.max(height_b) as f32;

    // 计算中心点
    let center_x = (points[0].x + points[2].x) / 2.0;
    let center_y = (points[0].y + points[2].y) / 2.0;

    // 扩展区域：我们将每个顶点从中心点向外扩展50%
    let scale_factor = 1.5; // 扩展比例为50%
    let expanded_points = points
        .iter()
        .map(|p| {
            Point2f::new(
                center_x + (p.x - center_x) * scale_factor,
                center_y + (p.y - center_y) * scale_factor,
            )
        })
        .collect::<Vec<Point2f>>();

    // 将扩展后的四个顶点存储到 src_points
    let mut expanded_src_points = opencv::types::VectorOfPoint2f::new();
    for point in &expanded_points {
        expanded_src_points.push(*point);
    }

    // 目标矩形的四个顶点，保持扩展后宽高比
    let mut dst_points = opencv::types::VectorOfPoint2f::new();
    dst_points.push(Point2f::new(0.0, 0.0));
    dst_points.push(Point2f::new(max_width * scale_factor, 0.0));
    dst_points.push(Point2f::new(max_width * scale_factor, max_height * scale_factor));
    dst_points.push(Point2f::new(0.0, max_height * scale_factor));

    // 计算透视变换矩阵
    let perspective_matrix = get_perspective_transform(&expanded_src_points, &dst_points, 0)?;

    // 透视变换后的图像
    let mut output = Mat::default();
    warp_perspective(
        image,
        &mut output,
        &perspective_matrix,
        Size::new((max_width * scale_factor) as i32, (max_height * scale_factor) as i32),
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


fn enhance_vertical_lines(gray_image: &Mat) -> opencv::Result<Mat> {
    // 1. 使用直方图均衡化来提高对比度
    let mut enhanced_image = Mat::default();
    equalize_hist(&gray_image, &mut enhanced_image)?;

    // 2. 使用 Sobel 算子在垂直方向上提取梯度 (dx=0, dy=1)
    let mut sobel_y = Mat::default();
    sobel(&enhanced_image, &mut sobel_y, CV_16S, 0, 1, 3, 1.0, 0.0, BORDER_DEFAULT)?;

    // 将结果转换为绝对值并转换为 CV_8U 类型
    let mut enhanced_sobel_y = Mat::default();
    convert_scale_abs(&sobel_y, &mut enhanced_sobel_y, 1.0, 0.0)?;

    // 3. 使用 Otsu 阈值法进行二值化处理
    let mut binary_image = Mat::default();
    threshold(&enhanced_sobel_y, &mut binary_image, 0.0, 255.0, THRESH_BINARY | THRESH_OTSU)?;

    // 4. 形态学操作（闭操作：膨胀后腐蚀），用于连接断开的条形码线条
    // let mut result = Mat::default();
    // let kernel = Mat::ones(3, 3, CV_8U)?; // 3x3 核心，用于形态学操作
    // let anchor = opencv::core::Point::new(-1, -1); // 使用默认的锚点（核中心）
    // let border = Scalar::default(); // 使用默认的边界值
    // morphology_ex(&binary_image, &mut result, MORPH_CLOSE, &kernel, anchor, 1, BORDER_DEFAULT, border)?;

    Ok(binary_image)
}

fn enhance_barcode_image(gray_image: &Mat) -> opencv::Result<Mat> {

    // 提高对比度
    let mut contrast_enhanced = Mat::default();
    imgproc::equalize_hist(&gray_image, &mut contrast_enhanced)?;

    // // 锐化图像
    // let mut sharpened = Mat::default();
    // let kernel = Mat::from_slice_2d(&[
    //     [-1.0, -1.0, -1.0],
    //     [-1.0, 9.0, -1.0],
    //     [-1.0, -1.0, -1.0],
    // ])?;
    // imgproc::filter_2d(&contrast_enhanced, &mut sharpened, CV_8U, &kernel, opencv::core::Point::new(-1, -1), 0.0, opencv::core::BORDER_DEFAULT)?;

    Ok(contrast_enhanced)
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
        let code_image = extract_and_expand(gray_image, &code_points).map_err(|e| Exception::new(0, &format!("Failed to extract barcode: {}", e)))?;
        imwrite(&format!("code_{}.png", i), &code_image, &Vector::new()).map_err(|e| Exception::new(0, &format!("Failed to save barcode: {}", e)))?;
        let enhance_mat = enhance_barcode_image(&code_image).map_err(|e| Exception::new(0, &format!("Failed to enhance barcode: {}", e)))?;
        imwrite(&format!("enhance_{}.png", i), &enhance_mat, &Vector::new()).map_err(|e| Exception::new(0, &format!("Failed to save enhanced barcode: {}", e)))?;
        let enhance_points = Vector::<Point2f>::from_slice(&[
            Point2f::new(0.0, 0.0),
            Point2f::new(code_image.cols() as f32, 0.0),
            Point2f::new(code_image.cols() as f32, code_image.rows() as f32),
            Point2f::new(0.0, code_image.rows() as f32),
        ]);
        let mut straight_code = Mat::default();
        let barcode = barcode_detector.decode(&code_image, &enhance_points, &mut straight_code).map_err(|e| Exception::new(0, &format!("Failed to decode barcode: {}", e)))?;
        results.push(CodeInfo{
            code: String::from_utf8(barcode).unwrap(),
            points: info_points,
            category: String::new(),
        });
    }
    Ok(results)
}