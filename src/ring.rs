/**
 * File: /src/ring.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 5:03:59 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int};
use opencv::core::{Point, Scalar, Size};
use opencv::imgproc;
use opencv::prelude::*;
use std::fs::{self, File};
use std::io::Write;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Ring {
    pub x1: c_double,
    pub y1: c_double,
    pub x2: c_double,
    pub y2: c_double,
    pub width: c_double,
    pub cx: c_double,
    pub cy: c_double,
    pub theta: c_double,
    pub ax: c_double,
    pub bx: c_double,
    pub ang_start: c_double,
    pub ang_end: c_double,
    pub wmin: c_double,
    pub wmax: c_double,
    pub full: c_int,
}

#[allow(unused)]
impl Ring {
    pub fn new(
        x1: c_double,
        y1: c_double,
        x2: c_double,
        y2: c_double,
        width: c_double,
        cx: c_double,
        cy: c_double,
        theta: c_double,
        ax: c_double,
        bx: c_double,
        ang_start: c_double,
        ang_end: c_double,
        wmin: c_double,
        wmax: c_double,
        full: c_int,
        label: c_int,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            width,
            cx,
            cy,
            theta,
            ax,
            bx,
            ang_start,
            ang_end,
            wmin,
            wmax,
            full,
        }
    }

    pub fn log_to_file(&self, file: &mut File) -> Result<(), std::io::Error> {
        writeln!(
            file,
            "Ring {}: center=({}, {}), axes=({}, {}), angle={}, startAngle={}, endAngle={}, full={}",
            0,
            self.cx,
            self.cy,
            self.ax,
            self.bx,
            self.theta,
            self.ang_start,
            self.ang_end,
            self.full
        )
    }

    pub fn draw(&self, img: &mut Mat) -> opencv::Result<()> {
        let color = Scalar::new(0.0, 255.0, 0.0, 0.0);
        let thickness = 2;

        // mkdir -p result
        fs::create_dir_all("result").unwrap();

        // if exist out_rust.txt then add argument to a new line
        // if not exist then create a new file and add argument to a new line
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("result/out_rust.txt")
            .unwrap();

        // match self.log_to_file(&mut file)
        match self.log_to_file(&mut file) {
            Ok(_) => {},
            Err(e) => eprintln!("Failed to log ring to file: {}", e),
        };

        if self.full != 0 {
            imgproc::ellipse(
                img,
                Point::new(self.cx as i32, self.cy as i32),
                Size::new(self.ax as i32, self.bx as i32),
                self.theta * 180.0 / std::f64::consts::PI,
                0.0,
                360.0,
                color,
                thickness,
                imgproc::LINE_8,
                0,
            )?;
        } else {
            imgproc::ellipse(
                img,
                Point::new(self.cx as i32, self.cy as i32),
                Size::new(self.ax as i32, self.bx as i32),
                self.theta * 180.0 / std::f64::consts::PI,
                self.ang_start * 180.0 / std::f64::consts::PI,
                self.ang_end * 180.0 / std::f64::consts::PI,
                color,
                thickness,
                imgproc::LINE_8,
                0,
            )?;
        }

        Ok(())
    }

    pub fn iou(&self, other: &Ring) -> f64 {
        // 创建两个椭圆的mask
        let size = Size::new(
            (self.cx.max(other.cx) * 2.0) as i32,
            (self.cy.max(other.cy) * 2.0) as i32,
        );
        let mut mask1 = Mat::zeros(size.height, size.width, opencv::core::CV_8UC1).unwrap().to_mat().unwrap();
        let mut mask2 = Mat::zeros(size.height, size.width, opencv::core::CV_8UC1).unwrap().to_mat().unwrap();
    
        // 绘制椭圆
        imgproc::ellipse(
            &mut mask1,
            Point::new(self.cx as i32, self.cy as i32),
            Size::new(self.ax as i32, self.bx as i32),
            self.theta * 180.0 / std::f64::consts::PI,
            self.ang_start * 180.0 / std::f64::consts::PI,
            self.ang_end * 180.0 / std::f64::consts::PI,
            Scalar::new(255.0, 0.0, 0.0, 0.0),
            -1,
            imgproc::LINE_8,
            0,
        ).unwrap();
    
        imgproc::ellipse(
            &mut mask2,
            Point::new(other.cx as i32, other.cy as i32),
            Size::new(other.ax as i32, other.bx as i32),
            other.theta * 180.0 / std::f64::consts::PI,
            other.ang_start * 180.0 / std::f64::consts::PI,
            other.ang_end * 180.0 / std::f64::consts::PI,
            Scalar::new(255.0, 0.0, 0.0, 0.0),
            -1,
            imgproc::LINE_8,
            0,
        ).unwrap();
    
        // 计算交集和并集
        let mut intersection = Mat::default();
        let mut union = Mat::default();
        opencv::core::bitwise_and(&mask1, &mask2, &mut intersection, &Mat::default()).unwrap();
        opencv::core::bitwise_or(&mask1, &mask2, &mut union, &Mat::default()).unwrap();
    
        // 计算非零像素数量
        let intersection_area = opencv::core::count_non_zero(&intersection).unwrap() as f64;
        let union_area = opencv::core::count_non_zero(&union).unwrap() as f64;
    
        // 计算IOU
        if union_area == 0.0 {
            0.0
        } else {
            intersection_area / union_area
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iou_identical_rings() {
        let ring1 = Ring {
            x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0,
            width: 1.0, cx: 0.5, cy: 0.5, theta: 0.0,
            ax: 0.5, bx: 0.5, ang_start: 0.0, ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0, wmax: 1.0, full: 1,
        };
        let ring2 = ring1.clone();
        assert_eq!(ring1.iou(&ring2), 1.0);
    }

    #[test]
    fn test_iou_non_overlapping_rings() {
        let ring1 = Ring {
            x1: 0.0, y1: 0.0, x2: 1.0, y2: 1.0,
            width: 1.0, cx: 0.5, cy: 0.5, theta: 0.0,
            ax: 0.5, bx: 0.5, ang_start: 0.0, ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0, wmax: 1.0, full: 1,
        };
        let ring2 = Ring {
            x1: 2.0, y1: 2.0, x2: 3.0, y2: 3.0,
            width: 1.0, cx: 2.5, cy: 2.5, theta: 0.0,
            ax: 0.5, bx: 0.5, ang_start: 0.0, ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0, wmax: 1.0, full: 1,
        };
        assert_eq!(ring1.iou(&ring2), 0.0);
    }

    #[test]
    fn test_iou_partially_overlapping_rings() {
        let ring1 = Ring {
            x1: 0.0, y1: 0.0, x2: 2.0, y2: 2.0,
            width: 1.0, cx: 1.0, cy: 1.0, theta: 0.0,
            ax: 1.0, bx: 1.0, ang_start: 0.0, ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0, wmax: 1.0, full: 1,
        };
        let ring2 = Ring {
            x1: 1.0, y1: 1.0, x2: 3.0, y2: 3.0,
            width: 1.0, cx: 2.0, cy: 2.0, theta: 0.0,
            ax: 1.0, bx: 1.0, ang_start: 0.0, ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0, wmax: 1.0, full: 1,
        };
        let iou = ring1.iou(&ring2);
        assert!(iou > 0.0 && iou < 1.0);
    }
}
