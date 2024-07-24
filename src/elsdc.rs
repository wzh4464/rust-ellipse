/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 9:28:43 pm
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
            let out_size =
                ((*PImageInt::from_ptr(out)).xsize * (*PImageInt::from_ptr(out)).ysize) as usize;
            let _ = Vec::from_raw_parts(out, out_size, out_size);
        }
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
