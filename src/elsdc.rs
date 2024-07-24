/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 11:23:43 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;

use crate::primitives::{Image, Primitive};
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
pub fn detect_primitives(
    image: &mut dyn Image,
    ell_out: &mut *mut Ring,
    ell_labels: &mut *mut c_int,
    ell_count: &mut c_int,
    out: &mut *mut c_int,
) -> Result<Vec<Box<dyn Primitive>>, Box<dyn std::error::Error>> {
    unsafe {
        let in_img = ImageDouble {
            data: image.as_mut_ptr(),
            xsize: image.width(),
            ysize: image.height(),
        };

        let mut out_data: Vec<c_int> = vec![0; (image.width() * image.height()) as usize];
        let out_data_ptr: *mut c_int = out_data.as_mut_ptr();

        let mut out_img = PImageInt {
            data: out_data_ptr,
            xsize: image.width(),
            ysize: image.height(),
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
            .open("result/elsdc_params_2.log").unwrap();
        let _ = writeln!(file, "{}", params_json);
        
        // save in_img to result/elsdc_in_img.png

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

        let mut primitives = Vec::new();
        for i in 0..*ell_count {
            let ring = &*(*ell_out).add(i as usize);
            primitives.push(Box::new(ring.clone()) as Box<dyn Primitive>);
        }

        *out = Box::into_raw(out_data.into_boxed_slice()) as *mut c_int;

        Ok(primitives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processing::OpenCVImage;

    #[test]
    fn test_detect_primitives() {
        let mut image = OpenCVImage::new(100, 100).unwrap();

        // 在图像中绘制一个圆
        for i in 0..100 {
            for j in 0..100 {
                let dx = (i as f64 - 50.0) / 25.0;
                let dy = (j as f64 - 50.0) / 25.0;
                if dx * dx + dy * dy < 1.1 && dx * dx + dy * dy > 0.9 {
                    image.set_pixel(i, j, 255.0).unwrap();
                }
            }
        }

        // image.save("result/test_data.png").unwrap();
        // save as pgm
        image.save("result/test_data.pgm").unwrap();

        let mut ell_out: *mut Ring = null_mut();
        let mut ell_labels: *mut c_int = null_mut();
        let mut ell_count: c_int = 0;
        let mut out: *mut c_int = null_mut();

        let primitives = detect_primitives(
            &mut image,
            &mut ell_out,
            &mut ell_labels,
            &mut ell_count,
            &mut out,
        )
        .unwrap();
        log::debug!("Detected {} primitives", primitives.len());
        assert!(!primitives.is_empty());

        let first_primitive = &primitives[0];
        if let Some(ring) = first_primitive.as_any().downcast_ref::<Ring>() {
            assert!((ring.cx - 50.0).abs() < 5.0);
            assert!((ring.cy - 50.0).abs() < 5.0);
            assert!((ring.ax - 25.0).abs() < 2.0);
        } else {
            panic!("First primitive is not a Ring");
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
