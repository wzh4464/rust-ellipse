/**
 * File: /build.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Tuesday, 23rd July 2024 11:41:59 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
extern crate pkg_config;

fn main() {
    // 创建 result 文件夹
    let result_dir = PathBuf::from("result");
    if !result_dir.exists() {
        fs::create_dir(&result_dir).expect("Failed to create result directory");
    }

    // 创建或打开日志文件
    let log_file_path = result_dir.join("build.log");
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Failed to open log file");

    // 定义日志记录宏
    macro_rules! log {
        ($($arg:tt)*) => ({
            writeln!(log_file, $($arg)*).expect("Failed to write to log file");
            writeln!(io::stderr(), $($arg)*).expect("Failed to write to stderr");
        })
    }

    // 输出所有环境变量到日志文件和 stderr
    log!("Environment Variables:");
    for (key, value) in env::vars() {
        log!("{}: {}", key, value);
    }

    // 确认 make 工具在系统路径中
    let make = env::var("MAKE").unwrap_or_else(|_| "make".to_string());

    // 运行 Makefile 编译 C 代码
    let status = Command::new(&make)
        .arg("shared")
        .current_dir("ELSDc_c")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        log!("Failed to build ELSDc_c library");
        panic!("Failed to build ELSDc_c library");
    }

    // lib extension is .so on linux and .dylib on macos
    let lib_ext = if cfg!(target_os = "macos") { "dylib" } else { "so" };

    // 移动 libelsdc.${lib_ext} 到当前文件夹
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let src = current_dir.join("ELSDc_c").join("libelsdc").with_extension(&lib_ext);
    let dst = current_dir.join("libelsdc").with_extension(&lib_ext);
    fs::rename(&src, &dst).expect("Failed to move libelsdc library to current directory");
    log!("Moved libelsdc.{} to current directory {}", lib_ext, current_dir.display());

    // 告诉 cargo 链接编译后的共享库
    println!("cargo:rustc-link-search=native={}", current_dir.display());
    log!("Link search path: {}", current_dir.display());
    println!("cargo:rustc-link-lib=dylib=elsdc");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", current_dir.display());

    // 确认 OpenCV 库
    let opencv = if cfg!(target_os = "macos") {
        pkg_config::Config::new().probe("opencv4").unwrap()
    } else {
        pkg_config::Config::new().probe("opencv4").unwrap_or_else(|_| pkg_config::Config::new().probe("opencv").unwrap())
    };

    log!("OpenCV libraries and paths:");
    for path in &opencv.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
        log!("Link search path: {}", path.display());
    }
    for lib in &opencv.libs {
        println!("cargo:rustc-link-lib={}", lib);
        log!("Link library: {}", lib);
    }
    for framework in &opencv.frameworks {
        println!("cargo:rustc-link-lib=framework={}", framework);
        log!("Link framework: {}", framework);
    }

    // 生成绑定代码
    let bindings = bindgen::Builder::default()
        .header("ELSDc_c/src/elsdc.h")
        .clang_arg("-IELSDc_c/src")
        .clang_args(opencv.include_paths.iter().map(|path| format!("-I{}", path.display())))
        .generate()
        .expect("Failed to generate bindings");

    // 输出绑定代码到文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");

    log!("Bindings generated and written to {}", out_path.display());
}
