/**
 * File: /src/ring.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Monday, 22nd July 2024 11:04:27 pm
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
use std::fs::File;
use std::io::Write;

#[repr(C)]
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

        // if exist out_rust.txt then add argument to a new line
        // if not exist then create a new file and add argument to a new line
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("out_rust.txt")
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
}
