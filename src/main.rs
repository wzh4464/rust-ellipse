/**
 * File: /src/main.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 18th July 2024 4:27:39 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

extern crate elsdc;
extern crate opencv;

use elsdc::elsdc::{detect_primitives, free_outputs};
use libc::c_double;
use opencv::core::Mat;
use opencv::core::Scalar;
use opencv::prelude::*;
use opencv::highgui;

fn main() {
    let mut ell_out: *mut elsdc::ring::Ring = std::ptr::null_mut();
    let mut ell_labels: *mut libc::c_int = std::ptr::null_mut();
    let mut ell_count: libc::c_int = 0;
    let mut out: *mut libc::c_int = std::ptr::null_mut();
    let mut in_data: [c_double; 100] = [0.0; 100]; // 示例输入数据
    let xsize: usize = 10;
    let ysize: usize = 10;

    unsafe {
        let result = detect_primitives(
            &mut ell_out,
            &mut ell_labels,
            &mut ell_count,
            &mut out,
            &mut in_data,
            xsize,
            ysize,
        );

        if result == 0 {
            println!("Detection successful!");
            // 使用结果
            for i in 0..ell_count as usize {
                let ring = ell_out.add(i).read();
                let label = ell_labels.add(i).read();
                println!(
                    "Ring {}: center=({}, {}), axes=({}, {}), angle={}, startAngle={}, endAngle={}, full={}",
                    label, ring.cx, ring.cy, ring.ax, ring.bx, ring.theta, ring.ang_start, ring.ang_end, ring.full
                );

                // 创建一个空白图像
                let mut img: opencv::core::Mat = Mat::new_rows_cols_with_default(ysize as i32, xsize as i32, opencv::core::CV_8UC3, Scalar::all(0.0)).unwrap();
                // 绘制检测到的椭圆或圆弧
                ring.draw(&mut img).unwrap();
                // 显示或保存图像
                highgui::imshow("Detected Ring", &img).unwrap();
                highgui::wait_key(0).unwrap();
            }
        } else {
            println!("Detection failed!");
        }

        free_outputs(ell_out, ell_labels, ell_count, out);
    }
}
