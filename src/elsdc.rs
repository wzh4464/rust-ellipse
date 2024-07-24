/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 11:14:59 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;

use crate::ring::Ring;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct ELSDcParams {
    in_img: ImageDoubleInfo,
    ell_count: i32,
    ell_out_ptr: usize,
    ell_labels_ptr: usize,
    poly_count: i32,
    poly_out_ptr: usize,
    poly_labels_ptr: usize,
    out_img: PImageIntInfo,
}

#[derive(Serialize, Deserialize)]
struct ImageDoubleInfo {
    data_ptr: usize,
    xsize: u32,
    ysize: u32,
    hash: u64,
}

#[derive(Serialize, Deserialize)]
struct PImageIntInfo {
    data_ptr: usize,
    xsize: u32,
    ysize: u32,
    hash: u64,
}

#[repr(C)]
pub struct ImageDouble {
    pub data: *mut c_double,
    pub xsize: c_uint,
    pub ysize: c_uint,
}

#[repr(C)]
#[derive(Debug)]
pub struct PImageInt {
    pub data: *mut c_int,
    pub xsize: c_uint,
    pub ysize: c_uint,
}

impl PImageInt {
    unsafe fn from_ptr<'a>(ptr: *mut c_int) -> &'a PImageInt {
        &*(ptr as *const PImageInt)
    }
}
extern "C" {
    fn ELSDc(
        in_img: *const ImageDouble,
        ell_count: *mut c_int,
        ell_out: *mut *mut Ring,
        ell_labels: *mut *mut c_int,
        poly_count: *mut c_int,
        poly_out: *mut *mut c_void,
        poly_labels: *mut *mut c_int,
        out: *mut PImageInt,
    );

    pub fn read_pgm_image_double(filename: *const libc::c_char) -> *mut ImageDouble;
    pub fn free_PImageDouble(image: *mut ImageDouble);
}

/// Detects ellipses and circular arcs in the given image data.
///
/// # Arguments
///
/// * `ell_out` - Pointer to store detected ellipses
/// * `ell_labels` - Pointer to store labels for detected ellipses
/// * `ell_count` - Pointer to store the count of detected ellipses
/// * `out` - Pointer to store output image data
/// * `in_data` - Input image data
/// * `xsize` - Width of the input image
/// * `ysize` - Height of the input image
///
/// # Safety
///
/// This function uses raw pointers and should be called carefully.
#[no_mangle]
pub extern "C" fn detect_primitives(
    ell_out: &mut *mut Ring,
    ell_labels: &mut *mut c_int,
    ell_count: &mut c_int,
    out: &mut *mut c_int,
    in_data: *mut c_double,
    xsize: c_uint,
    ysize: c_uint,
) -> c_int {
    unsafe {
        let in_img = ImageDouble {
            data: in_data,
            xsize,
            ysize,
        };

        // Create a zero-initialized i32 array and get its raw pointer
        let mut out_data: Vec<c_int> = vec![0; (xsize * ysize) as usize];
        let out_data_ptr: *mut c_int = out_data.as_mut_ptr();

        let mut out_img = PImageInt {
            data: out_data_ptr,
            xsize,
            ysize,
        };

        let mut poly_count: c_int = 0;
        let mut poly_out: *mut c_void = null_mut();
        let mut poly_labels: *mut c_int = null_mut();

        let params = ELSDcParams {
            in_img: ImageDoubleInfo {
                data_ptr: in_img.data as usize,
                xsize: in_img.xsize,
                ysize: in_img.ysize,
                hash: calculate_hash(in_img.data, in_img.xsize * in_img.ysize),
            },
            ell_count: *ell_count,
            ell_out_ptr: *ell_out as usize,
            ell_labels_ptr: *ell_labels as usize,
            poly_count,
            poly_out_ptr: poly_out as usize,
            poly_labels_ptr: poly_labels as usize,
            out_img: PImageIntInfo {
                data_ptr: out_img.data as usize,
                xsize: out_img.xsize,
                ysize: out_img.ysize,
                hash: calculate_hash_int(out_img.data, out_img.xsize * out_img.ysize),
            },
        };

        // 序列化参数
        let params_json = serde_json::to_string(&params).unwrap();

        // 写入日志文件
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("result/elsdc_params_1.log").unwrap();
        let _ = writeln!(file, "{}", params_json);
        
        ELSDc(
            &in_img,
            ell_count,
            ell_out,
            ell_labels,
            &mut poly_count,
            &mut poly_out,
            &mut poly_labels,
            &mut out_img,
        );

        let boxed_out_data = out_data.into_boxed_slice();
        *out = Box::into_raw(boxed_out_data) as *mut c_int;

        0 // Success
    }
}

#[no_mangle]
pub extern "C" fn free_outputs(
    ell_out: *mut Ring,
    ell_labels: *mut c_int,
    ell_count: c_int,
    out: *mut c_int,
) {
    unsafe {
        if !ell_out.is_null() {
            let _ = Vec::from_raw_parts(ell_out, ell_count as usize, ell_count as usize);
        }
        if !ell_labels.is_null() {
            let _ = Vec::from_raw_parts(ell_labels, ell_count as usize, ell_count as usize);
        }
        if !out.is_null() {
            let out_size = ((*PImageInt::from_ptr(out)).xsize * (*PImageInt::from_ptr(out)).ysize) as usize;
            let _ = Vec::from_raw_parts(out, out_size, out_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::{core, imgcodecs, prelude::*};
    use std::fs;
    use std::ptr::null_mut;

    #[test]
    fn test_detect_primitives() {
        // 创建一个简单的测试图像
        let mut test_data: Vec<c_double> = vec![0.0; 100 * 100];
        // 在测试数据中添加一个thickness为 1 的简单的圆形
        for i in 0..100 {
            for j in 0..100 {
                let dx = (i as f64 - 50.0) / 25.0;
                let dy = (j as f64 - 50.0) / 25.0;
                if dx * dx + dy * dy < 1.0 {
                    if dx * dx + dy * dy > 0.9 {
                        test_data[i * 100 + j] = 255.0;
                    } else {
                        test_data[i * 100 + j] = 128.0;
                    }
                } else {
                    test_data[i * 100 + j] = 0.0;
                }
            }
        }

        // 将测试数据转换为 OpenCV Mat
        let mut test_mat =
            Mat::new_rows_cols_with_default(100, 100, core::CV_64F, core::Scalar::all(0.0))
                .expect("Failed to create Mat");

        // 填充测试数据
        for i in 0..100 {
            for j in 0..100 {
                *test_mat.at_2d_mut::<f64>(i as i32, j as i32).unwrap() = test_data[i * 100 + j];
            }
        }

        // 归一化数据到 0-255 范围
        let mut normalized_mat = Mat::default();
        core::normalize(
            &test_mat,
            &mut normalized_mat,
            0.0,
            255.0,
            core::NORM_MINMAX,
            core::CV_8U,
            &core::no_array(),
        )
        .unwrap();

        // 确保 result 目录存在
        fs::create_dir_all("result").expect("Failed to create result directory");

        // 保存测试图像
        imgcodecs::imwrite(
            "result/test_data.png",
            &normalized_mat,
            &core::Vector::new(),
        )
        .expect("Failed to write image");

        let mut ell_out: *mut Ring = null_mut();
        let mut ell_labels: *mut c_int = null_mut();
        let mut ell_count: c_int = 0;
        let mut out: *mut c_int = null_mut();

        unsafe {
            let result = detect_primitives(
                &mut ell_out,
                &mut ell_labels,
                &mut ell_count,
                &mut out,
                test_data.as_mut_ptr(),
                100,
                100,
            );

            assert_eq!(result, 0); // 检查函数是否成功执行
            assert!(ell_count > 0); // 检查是否检测到至少一个椭圆

            // 使用引用来安全地访问 ell_out 指向的值
            let first_ellipse = &*ell_out;
            assert!((first_ellipse.cx - 50.0).abs() < 1.0); // 中心x坐标应接近50
            assert!((first_ellipse.cy - 50.0).abs() < 1.0); // 中心y坐标应接近50
            assert!((first_ellipse.ax - 25.0).abs() < 1.0); // 主轴长度应接近25

            // 清理内存
            free_outputs(ell_out, ell_labels, ell_count, out);
        }
    }
}

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash(data: *const c_double, len: c_uint) -> u64 {
    let mut hasher = DefaultHasher::new();
    unsafe {
        let byte_slice = std::slice::from_raw_parts(
            data as *const u8,
            len as usize * std::mem::size_of::<c_double>()
        );
        byte_slice.hash(&mut hasher);
    }
    hasher.finish()
}

fn calculate_hash_int(data: *const c_int, len: c_uint) -> u64 {
    let mut hasher = DefaultHasher::new();
    unsafe {
        let byte_slice = std::slice::from_raw_parts(
            data as *const u8,
            len as usize * std::mem::size_of::<c_int>()
        );
        byte_slice.hash(&mut hasher);
    }
    hasher.finish()
}
