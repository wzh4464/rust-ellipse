/**
 * File: /src/image_processing.rs
 * Created Date: Wednesday, July 24th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 9:05:22 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use crate::primitives::Image;
use opencv::core::{Mat, MatTraitConst, MatTrait, Vector};
use opencv::imgcodecs;
use libc::c_double;
use std::any::Any;

pub struct OpenCVImage {
    pub mat: Mat,
}

impl OpenCVImage {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mat = Mat::new_rows_cols_with_default(
            height as i32, 
            width as i32, 
            opencv::core::CV_64F, 
            opencv::core::Scalar::all(0.0)
        )?;
        Ok(Self { mat })
    }

    pub fn save(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        imgcodecs::imwrite(filename, &self.mat, &Vector::new())?;
        Ok(())
    }
}

impl Image for OpenCVImage {
    fn width(&self) -> u32 {
        self.mat.cols() as u32
    }

    fn height(&self) -> u32 {
        self.mat.rows() as u32
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: f64) -> Result<(), Box<dyn std::error::Error>> {
        *self.mat.at_2d_mut::<f64>(y as i32, x as i32)? = value;
        Ok(())
    }

    fn get_pixel(&self, x: u32, y: u32) -> Result<f64, Box<dyn std::error::Error>> {
        Ok(*self.mat.at_2d::<f64>(y as i32, x as i32)?)
    }

    fn as_ptr(&self) -> *const c_double {
        self.mat.data() as *const c_double
    }

    fn as_mut_ptr(&mut self) -> *mut f64 {
        self.mat.data_mut() as *mut f64
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
