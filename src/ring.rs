/**
 * File: /src/ring.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 11:38:10 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::{c_double, c_int};
use opencv::core::{Point, Scalar, Size};
use std::fs::{self, File};
use std::io::Write;
use opencv::{core, imgproc, prelude::*};
use crate::primitives::{Primitive, Image};
use crate::ElsdcError;
use crate::image_processing::OpenCVImage;

impl Primitive for Ring {
    fn draw(&self, image: &mut dyn Image) -> Result<(), Box<dyn std::error::Error>> {
        let opencv_image = image.as_any_mut().downcast_mut::<OpenCVImage>()
            .ok_or_else(|| Box::new(ElsdcError::ImageConversionError("Failed to downcast Image to OpenCVImage".to_string())))?;
    
        let mut mat = opencv_image.mat.clone();

        let center = core::Point::new(self.cx as i32, self.cy as i32);
        let axes = core::Size::new(self.ax as i32, self.bx as i32);
        let color = core::Scalar::new(255.0, 255.0, 255.0, 0.0); // 白色
        let thickness = 2;

        if self.full != 0 {
            imgproc::ellipse(
                &mut mat,
                center,
                axes,
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
                &mut mat,
                center,
                axes,
                self.theta * 180.0 / std::f64::consts::PI,
                self.ang_start * 180.0 / std::f64::consts::PI,
                self.ang_end * 180.0 / std::f64::consts::PI,
                color,
                thickness,
                imgproc::LINE_8,
                0,
            )?;
        }

        // 将修改后的 mat 复制回 OpenCVImage
        opencv_image.mat = mat;

        Ok(())
    }

    fn to_string(&self) -> String {
        format!("Ring: center=({}, {}), axes=({}, {}), angle={}", 
                self.cx, self.cy, self.ax, self.bx, self.theta)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represents an ellipse or circular arc detected in an image.
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
    pub fn log_to_file(&self, file: &mut File) -> Result<(), ElsdcError> {
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
        ).map_err(ElsdcError::IoError)?;
        Ok(())
    }

    pub fn draw(&self, img: &mut Mat) -> Result<(), ElsdcError> {
        let color = Scalar::new(0.0, 255.0, 0.0, 0.0);
        let thickness = 2;

        fs::create_dir_all("result")?;

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("result/out_rust.txt")
            .map_err(ElsdcError::IoError)?;

        self.log_to_file(&mut file)?;

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
            )
            .map_err(ElsdcError::OpenCVError)?;
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
            )
            .map_err(ElsdcError::OpenCVError)?;
        }

        Ok(())
    }

    pub fn iou(&self, other: &Ring) -> f64 {
        // 创建两个椭圆的mask
        let size = Size::new(
            (self.cx.max(other.cx) * 2.0) as i32,
            (self.cy.max(other.cy) * 2.0) as i32,
        );
        let mut mask1 = Mat::zeros(size.height, size.width, opencv::core::CV_8UC1)
            .unwrap()
            .to_mat()
            .unwrap();
        let mut mask2 = Mat::zeros(size.height, size.width, opencv::core::CV_8UC1)
            .unwrap()
            .to_mat()
            .unwrap();

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
        )
        .unwrap();

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
        )
        .unwrap();

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
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
            width: 1.0,
            cx: 0.5,
            cy: 0.5,
            theta: 0.0,
            ax: 0.5,
            bx: 0.5,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0,
            wmax: 1.0,
            full: 1,
        };
        let ring2 = ring1.clone();
        assert_eq!(ring1.iou(&ring2), 1.0);
    }

    #[test]
    fn test_iou_non_overlapping_rings() {
        let ring1 = Ring {
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
            width: 1.0,
            cx: 0.5,
            cy: 0.5,
            theta: 0.0,
            ax: 0.5,
            bx: 0.5,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0,
            wmax: 1.0,
            full: 1,
        };
        let ring2 = Ring {
            x1: 2.0,
            y1: 2.0,
            x2: 3.0,
            y2: 3.0,
            width: 1.0,
            cx: 2.5,
            cy: 2.5,
            theta: 0.0,
            ax: 0.5,
            bx: 0.5,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0,
            wmax: 1.0,
            full: 1,
        };
        assert_eq!(ring1.iou(&ring2), 0.0);
    }

    #[test]
    fn test_iou_partially_overlapping_rings() {
        let ring1 = Ring {
            x1: 0.0,
            y1: 0.0,
            x2: 2.0,
            y2: 2.0,
            width: 1.0,
            cx: 1.0,
            cy: 1.0,
            theta: 0.0,
            ax: 1.0,
            bx: 1.0,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0,
            wmax: 1.0,
            full: 1,
        };
        let ring2 = Ring {
            x1: 1.0,
            y1: 1.0,
            x2: 3.0,
            y2: 3.0,
            width: 1.0,
            cx: 2.0,
            cy: 2.0,
            theta: 0.0,
            ax: 1.0,
            bx: 1.0,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 0.0,
            wmax: 1.0,
            full: 1,
        };
        let iou = ring1.iou(&ring2);
        assert!(iou > 0.0 && iou < 1.0);
    }

    #[test]
    fn test_ring_draw() {
        let ring = Ring {
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 100.0,
            width: 2.0,
            cx: 50.0,
            cy: 50.0,
            theta: 0.0,
            ax: 25.0,
            bx: 25.0,
            ang_start: 0.0,
            ang_end: 2.0 * std::f64::consts::PI,
            wmin: 1.0,
            wmax: 3.0,
            full: 1,
        };

        let mut img = Mat::new_rows_cols_with_default(100, 100, opencv::core::CV_8UC3, Scalar::all(0.0)).unwrap();
        
        ring.draw(&mut img).unwrap();

        // 检查图像中是否有非黑色像素（椭圆被绘制）
        let mut has_non_black = false;
        for i in 0..100 {
            for j in 0..100 {
                let pixel = img.at_2d::<opencv::core::Vec3b>(i, j).unwrap();
                if pixel[0] != 0 || pixel[1] != 0 || pixel[2] != 0 {
                    has_non_black = true;
                    break;
                }
            }
            if has_non_black {
                break;
            }
        }
        assert!(has_non_black);

        // 可选：保存图像以进行视觉检查
        // 确保 result 目录存在
        fs::create_dir_all("result").expect("Failed to create result directory");
        opencv::imgcodecs::imwrite("result/test_ring_draw.png", &img, &opencv::core::Vector::new()).expect("Failed to write image");
    }
}
