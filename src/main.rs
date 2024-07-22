extern crate elsdc;
extern crate opencv;

use elsdc::elsdc::{detect_primitives, free_outputs};
use elsdc::elsdc::read_pgm_image_double;
use elsdc::elsdc::free_PImageDouble;
use libc::c_uint;
use opencv::core::Mat;
use opencv::core::Scalar;
use opencv::core::CV_8UC3;
use opencv::core::Vector;
use std::env;
use std::ffi::CString;

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

    let mut ell_out: *mut elsdc::ring::Ring = std::ptr::null_mut();
    let mut ell_labels: *mut libc::c_int = std::ptr::null_mut();
    let mut ell_count: libc::c_int = 0;
    let mut out: *mut libc::c_int = std::ptr::null_mut();

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
            // 使用结果
            for i in 0..ell_count as c_uint {
                let ring = ell_out.add(i as usize).read();
                let label = ell_labels.add(i as usize).read();
                // println!(
                //     "Ring {}: center=({}, {}), axes=({}, {}), angle={}, startAngle={}, endAngle={}, full={}",
                //     label, ring.cx, ring.cy, ring.ax, ring.bx, ring.theta, ring.ang_start, ring.ang_end, ring.full
                // );

                // 创建一个空白图像
                let mut img: Mat = Mat::new_rows_cols_with_default(ysize as i32, xsize as i32, CV_8UC3, Scalar::all(0.0)).unwrap();
                // 绘制检测到的椭圆或圆弧
                ring.draw(&mut img).unwrap();
                // 保存图像
                let output_path = format!("result/output_ring_{}.png", i);
                let params = Vector::new();
                opencv::imgcodecs::imwrite(&output_path, &img, &params).unwrap();
                // println!("Saved detected ring image to {}", output_path);
            }
        } else {
            println!("Detection failed!");
        }

        free_outputs(ell_out, ell_labels, ell_count, out);
        free_PImageDouble(img_double);
    }
}
