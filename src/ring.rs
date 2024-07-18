/**
 * File: /src/ring.rs
 * Created Date: Thursday, July 18th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 18th July 2024 4:23:02 pm
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

    pub fn draw(&self, img: &mut Mat) -> opencv::Result<()> {
        let color = Scalar::new(0.0, 255.0, 0.0, 0.0);
        let thickness = 2;

        if self.full != 0 {
            imgproc::ellipse(
                img,
                Point::new(self.cx as i32, self.cy as i32),
                Size::new(self.ax as i32, self.bx as i32),
                self.theta,
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
                self.theta,
                self.ang_start,
                self.ang_end,
                color,
                thickness,
                imgproc::LINE_8,
                0,
            )?;
        }

        Ok(())
    }
}
