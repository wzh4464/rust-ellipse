/**
 * File: /src/elsdc.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 18th July 2024 4:22:46 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int};
use std::ffi::c_void;
use std::ptr::null_mut;

use crate::ring::Ring;

#[repr(C)]
pub struct ImageDouble {
    data: *mut c_double,
    xsize: usize,
    ysize: usize,
}

#[repr(C)]
pub struct PImageInt {
    data: *mut c_int,
    xsize: usize,
    ysize: usize,
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
}

#[no_mangle]
pub extern "C" fn detect_primitives(
    ell_out: &mut *mut Ring,
    ell_labels: &mut *mut c_int,
    ell_count: &mut c_int,
    out: &mut *mut c_int,
    in_data: &mut [c_double],
    xsize: usize,
    ysize: usize,
) -> c_int {
    unsafe {
        let in_img = ImageDouble {
            data: in_data.as_mut_ptr(),
            xsize,
            ysize,
        };
        let mut out_img = PImageInt {
            data: null_mut(),
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
        0 // 成功
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
            Box::from_raw(ell_out);
            Box::from_raw(ell_labels);
        }
        if !out.is_null() {
            Box::from_raw(out);
        }
    }
}
