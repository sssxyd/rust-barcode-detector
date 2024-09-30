use std::fs;
use std::path::Path;
use std::env;


fn get_windows_dlls() -> (String, Vec<String>) {
    let dlls = vec![
        "concrt140.dll".to_string(),
        "msvcp140.dll".to_string(),
        "vcruntime140.dll".to_string(),
    ];
    ("C:/Windows/System32".to_string(), dlls)
}

fn get_opencv_dlls() -> (String, Vec<String>) {
    let dlls = vec![
        "opencv_core4.dll".to_string(),
        "opencv_calib3d4.dll".to_string(),
        "opencv_dnn4.dll".to_string(),
        "opencv_features2d4.dll".to_string(),
        "opencv_flann4.dll".to_string(),
        "opencv_imgcodecs4.dll".to_string(),
        "opencv_imgproc4.dll".to_string(),
        "opencv_objdetect4.dll".to_string(),
    ];
    ("E:/packages/vcpkg/installed/x64-windows/bin".to_string(), dlls)
}

fn get_opencv_debug_dlls() -> (String, Vec<String>) {
    let dlls = vec![
        "opencv_core4d.dll".to_string(),
        "opencv_calib3d4d.dll".to_string(),
        "opencv_dnn4d.dll".to_string(),
        "opencv_features2d4d.dll".to_string(),
        "opencv_flann4d.dll".to_string(),
        "opencv_imgcodecs4d.dll".to_string(),
        "opencv_imgproc4d.dll".to_string(),
        "opencv_objdetect4d.dll".to_string(),
    ];
    ("E:/packages/vcpkg/packages/opencv4_x64-windows/debug/bin".to_string(), dlls)
}

fn get_abseil_dll(is_debug: bool) -> (String, Vec<String>) {
    let dlls = vec![
        "abseil_dll.dll".to_string(),
    ];
    let path = if is_debug {
        "E:/packages/vcpkg/packages/abseil_x64-windows/debug/bin".to_string()
    } else {
        "E:/packages/vcpkg/packages/abseil_x64-windows/bin".to_string()
    };
    (path, dlls)
}

fn get_jpeg62_dll(is_debug: bool) -> (String, Vec<String>) {
    let dlls = vec![
        "jpeg62.dll".to_string(),
    ];
    let path = if is_debug {
        "E:/packages/vcpkg/packages/libjpeg-turbo_x64-windows/debug/bin".to_string()
    } else {
        "E:/packages/vcpkg/packages/libjpeg-turbo_x64-windows/bin".to_string()
    };
    (path, dlls)
}

fn get_protobuf_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/protobuf_x64-windows/bin".to_string(), vec![
        "libprotobuf.dll".to_string(),
    ])
}

fn get_protobuf_debug_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/protobuf_x64-windows/debug/bin".to_string(), vec![
        "libprotobufd.dll".to_string(),
    ])
}

fn get_liblzma_dll(is_debug: bool) -> (String, Vec<String>) {
    let path = if is_debug {
        "E:/packages/vcpkg/packages/liblzma_x64-windows/debug/bin".to_string()
    } else {
        "E:/packages/vcpkg/packages/liblzma_x64-windows/bin".to_string()
    };
    (path, vec![
        "liblzma.dll".to_string(),
    ])
}

fn get_libpng16_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/libpng_x64-windows/bin".to_string(),  vec![
        "libpng16.dll".to_string(),
    ])
}

fn get_libpng16_debug_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/libpng_x64-windows/debug/bin".to_string(), vec![
        "libpng16d.dll".to_string(),
    ])
}

fn get_libwebp_dll(is_debug: bool) -> (String, Vec<String>) {
    let path = if is_debug {
        "E:/packages/vcpkg/packages/libwebp_x64-windows/debug/bin".to_string()
    } else {
        "E:/packages/vcpkg/packages/libwebp_x64-windows/bin".to_string()
    };
    (path, vec![
        "libwebp.dll".to_string(),
        "libwebpdecoder.dll".to_string(),
        "libsharpyuv.dll".to_string(),
    ])
}

fn get_tiff_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/tiff_x64-windows/bin".to_string(), vec![
        "tiff.dll".to_string(),
    ])
}

fn get_tiff_debug_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/tiff_x64-windows/debug/bin".to_string(), vec![
        "tiffd.dll".to_string(),
    ])
}

fn get_zlib_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/zlib_x64-windows/bin".to_string(), vec![
        "zlib1.dll".to_string(),
    ])
}

fn get_zlib_debug_dll() -> (String, Vec<String>) {
    ("E:/packages/vcpkg/packages/zlib_x64-windows/debug/bin".to_string(), vec![
        "zlibd1.dll".to_string(),
    ])
}

fn copy_dlls(path: &str, dlls: &Vec<String>, dest_path: &Path) {
    for dll in dlls {
        let src_path = format!("{}/{}", path, dll);
        let dst_path = dest_path.join(dll);

        if !dst_path.exists() {
            fs::copy(src_path, &dst_path).expect(&format!("Failed to copy DLL file: {}", dll));
            println!("Copied {} to {}", dll, dst_path.display());
        } else {
            println!("{} already exists, skipping copy", dst_path.display());
        }
    }
}

fn main() {
    // 设置 OpenCV 库的路径
    println!("cargo:rustc-link-search=native=E:/packages/vcpkg/installed/x64-windows/lib");

    // 指定要链接的 OpenCV 库，列出你需要的模块
    println!("cargo:rustc-link-lib=dylib=opencv_core4");
    println!("cargo:rustc-link-lib=dylib=opencv_imgproc4");
    println!("cargo:rustc-link-lib=dylib=opencv_dnn4");
    println!("cargo:rustc-link-lib=dylib=opencv_imgcodecs4");
    println!("cargo:rustc-link-lib=dylib=opencv_objdetect4");

    // 设置 OpenCV 头文件的路径（对于 C++ 绑定）
    println!("cargo:include=E:/packages/vcpkg/installed/x64-windows/include");

    // 设置 OPENCV_DIR 环境变量
    env::set_var("OPENCV_DIR", "E:/packages/vcpkg/installed/x64-windows");

    // 获取当前构建配置（debug 或 release）
    let profile = if env::var("PROFILE").unwrap_or_else(|_| "debug".to_string()) == "release" {
        "release"
    } else {
        "debug"
    };

    // 获取目标目录
    let out_dir = "./target";
    let dest_path = Path::new(&out_dir).join(profile);

    let is_debug = profile == "debug";

    let windows_dlls = get_windows_dlls();
    copy_dlls(&windows_dlls.0, &windows_dlls.1, &dest_path);

    let opencv_dlls = get_opencv_dlls();
    copy_dlls(&opencv_dlls.0, &opencv_dlls.1, &dest_path);

    let opencv_debug_dlls = get_opencv_debug_dlls();
    copy_dlls(&opencv_debug_dlls.0, &opencv_debug_dlls.1, &dest_path);

    let abseil_dlls = get_abseil_dll(is_debug);
    copy_dlls(&abseil_dlls.0, &abseil_dlls.1, &dest_path);

    let jpeg62_dlls = get_jpeg62_dll(is_debug);
    copy_dlls(&jpeg62_dlls.0, &jpeg62_dlls.1, &dest_path);

    let protobuf_dlls = get_protobuf_dll();
    copy_dlls(&protobuf_dlls.0, &protobuf_dlls.1, &dest_path);

    let protobuf_debug_dlls = get_protobuf_debug_dll();
    copy_dlls(&protobuf_debug_dlls.0, &protobuf_debug_dlls.1, &dest_path);

    let liblzma_dlls = get_liblzma_dll(is_debug);
    copy_dlls(&liblzma_dlls.0, &liblzma_dlls.1, &dest_path);

    let libpng16_dlls = get_libpng16_dll();
    copy_dlls(&libpng16_dlls.0, &libpng16_dlls.1, &dest_path);

    let libpng16_debug_dlls = get_libpng16_debug_dll();
    copy_dlls(&libpng16_debug_dlls.0, &libpng16_debug_dlls.1, &dest_path);

    let libwebp_dlls = get_libwebp_dll(is_debug);
    copy_dlls(&libwebp_dlls.0, &libwebp_dlls.1, &dest_path);

    let tiff_dlls = get_tiff_dll();
    copy_dlls(&tiff_dlls.0, &tiff_dlls.1, &dest_path);

    let tiff_debug_dlls = get_tiff_debug_dll();
    copy_dlls(&tiff_debug_dlls.0, &tiff_debug_dlls.1, &dest_path);

    let zlib_dlls = get_zlib_dll();
    copy_dlls(&zlib_dlls.0, &zlib_dlls.1, &dest_path);

    let zlib_debug_dlls = get_zlib_debug_dll();
    copy_dlls(&zlib_debug_dlls.0, &zlib_debug_dlls.1, &dest_path);

    // 输出调试信息以确认路径是否正确
    println!("cargo:rerun-if-changed=build.rs");
}