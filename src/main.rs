extern crate elsdc;
extern crate opencv;

use elsdc::elsdc::{detect_primitives, free_PImageDouble, free_outputs, read_pgm_image_double};
use libc::c_uint;
use opencv::core::{Mat, Scalar, Vector, CV_8UC3};
use opencv::imgcodecs::{self};
use opencv::prelude::*;
use std::env;
use std::ffi::CString;
use std::ptr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_file>", args[0]);
        std::process::exit(1);
    }

    let filename = CString::new(args[1].clone()).unwrap();
    let img_double = unsafe { read_pgm_image_double(filename.as_ptr()) };

    if img_double.is_null() {
        eprintln!("Failed to read PGM image");
        std::process::exit(1);
    }

    let xsize = unsafe { (*img_double).xsize };
    let ysize = unsafe { (*img_double).ysize };

    let mut ell_out: *mut elsdc::ring::Ring = ptr::null_mut();
    let mut ell_labels: *mut libc::c_int = ptr::null_mut();
    let mut ell_count: libc::c_int = 0;
    let mut out: *mut libc::c_int = ptr::null_mut();

    unsafe {
        let result = detect_primitives(
            &mut ell_out,
            &mut ell_labels,
            &mut ell_count,
            &mut out,
            std::slice::from_raw_parts_mut((*img_double).data, (xsize * ysize) as usize),
            xsize,
            ysize,
        );

        if result == 0 {
            println!("Detection successful!");

            // Create a blank image to draw ellipses
            let mut img_all_ellipses = Mat::new_rows_cols_with_default(
                ysize as i32,
                xsize as i32,
                CV_8UC3,
                Scalar::all(0.0),
            )
            .unwrap();

            // Copy the image data to the Mat
            let img_data = img_all_ellipses.data_mut();
            for i in 0..(xsize * ysize) as usize {
                let pixel_value = (*img_double).data.add(i).read() as u8;
                let offset = i * 3;
                *img_data.offset(offset as isize) = pixel_value; // Blue channel
                *img_data.offset(offset as isize + 1) = pixel_value; // Green channel
                *img_data.offset(offset as isize + 2) = pixel_value; // Red channel
            }

            // 使用结果
            for i in 0..ell_count as c_uint {
                let ring = ell_out.add(i as usize).read();
                // let label = ell_labels.add(i as usize).read();

                // 绘制检测到的椭圆或圆弧
                ring.draw(&mut img_all_ellipses).unwrap();

                // 如果需要保存每个单独的椭圆，可以取消注释以下代码
                // 创建一个空白图像
                // let mut img: Mat = Mat::new_rows_cols_with_default(ysize as i32, xsize as i32, CV_8UC3, Scalar::all(0.0)).unwrap();
                // 绘制检测到的椭圆或圆弧
                // ring.draw(&mut img).unwrap();
                // 保存图像
                // let output_path = format!("result/output_ring_{}.png", i);
                // let params = Vector::new();
                // imgcodecs::imwrite(&output_path, &img, &params).unwrap();
            }

            // 保存包含所有椭圆的图像
            let output_path_all = "result/output_all_rings.png";
            let params = Vector::new();
            imgcodecs::imwrite(&output_path_all, &img_all_ellipses, &params).unwrap();
            println!("Saved detected rings image to {}", output_path_all);
        } else {
            println!("Detection failed!");
        }

        free_outputs(ell_out, ell_labels, ell_count, out);
        free_PImageDouble(img_double);
    }
}
