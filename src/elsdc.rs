/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Monday, 22nd July 2024 11:09:32 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int, c_uint, c_void};
use std::ptr::null_mut;

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

        // Ensure out_data is not deallocated
        std::mem::forget(out_data);

        let mut out_img = PImageInt {
            data: out_data_ptr,
            xsize,
            ysize,
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

        *out = out_img.data;
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
    let _ = ell_count;
    unsafe {
        if !ell_out.is_null() {
            let _ = Box::from_raw(ell_out);
            let _ = Box::from_raw(ell_labels);
        }
        if !out.is_null() {
            let _ = Box::from_raw(out);
        }
    }
}
