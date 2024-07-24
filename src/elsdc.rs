/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 25th July 2024 12:15:01 am
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
use crate::{ElsdcError, OpenCVImage};

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

pub fn detect_primitives_on_real_image(image_path: &str) -> Result<Vec<Box<dyn Primitive>>, ElsdcError> {
    let pgm_filename = crate::pgm::ensure_pgm_image(image_path)?;
    let cstring_filename = std::ffi::CString::new(pgm_filename.clone())
        .map_err(|e| ElsdcError::DetectionError(e.to_string()))?;
    
    let img_double = unsafe { read_pgm_image_double(cstring_filename.as_ptr()) };
    if img_double.is_null() {
        return Err(ElsdcError::ImageReadError("Failed to read PGM image".into()));
    }

    let mut image = OpenCVImage::try_from(img_double)?;
    
    let mut ell_out: *mut Ring = std::ptr::null_mut();
    let mut ell_labels: *mut c_int = std::ptr::null_mut();
    let mut ell_count: c_int = 0;
    let mut out: *mut c_int = std::ptr::null_mut();

    let primitives = detect_primitives(
        &mut image,
        &mut ell_out,
        &mut ell_labels,
        &mut ell_count,
        &mut out,
    )?;

    unsafe {
        free_PImageDouble(img_double);
    }

    Ok(primitives)
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

    #[test]
    fn test_detect_primitives_on_real_image() {
        let image_path = "ELSDc_c/Dataset4_mydataset/043_0011.jpg";
        
        if !std::path::Path::new(image_path).exists() {
            // 如果图像不存在，测试通过
            return;
        }

        let primitives = detect_primitives_on_real_image(image_path).unwrap();
        assert_eq!(primitives.len(), 46, "Expected to find 46 primitives");
    }
}
