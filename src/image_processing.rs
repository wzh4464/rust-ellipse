use crate::pgm::ensure_pgm_image;
/**
 * File: /src/image_processing.rs
 * Created Date: Wednesday, July 24th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 25th July 2024 12:16:16 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use crate::primitives::Image;
use crate::ElsdcError;
use opencv::core::{Mat, MatTraitConst, MatTrait, Vector};
use opencv::imgcodecs;
use libc::c_double;
use std::any::Any;
use crate::elsdc::ImageDouble;
use std::convert::TryFrom;

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

    pub fn save(&self, path: &str) -> Result<(), ElsdcError> {
        let mut mat_8u = Mat::default();
        self.mat.convert_to(&mut mat_8u, opencv::core::CV_8U, 255.0, 0.0)
            .map_err(|e| ElsdcError::OpenCVError(e))?;

        imgcodecs::imwrite(path, &mat_8u, &Vector::new())
            .map_err(|e| ElsdcError::OpenCVError(e))?;

        Ok(())
    }
}

impl TryFrom<*mut ImageDouble> for OpenCVImage {
    type Error = ElsdcError;

    fn try_from(img_double: *mut ImageDouble) -> Result<Self, Self::Error> {
        unsafe {
            if img_double.is_null() {
                return Err(ElsdcError::ImageConversionError("Null pointer provided".into()));
            }

            let xsize = (*img_double).xsize as i32;
            let ysize = (*img_double).ysize as i32;
            
            let mut mat = Mat::new_rows_cols_with_default(
                ysize, 
                xsize, 
                opencv::core::CV_64F, 
                opencv::core::Scalar::all(0.0)
            ).map_err(|e| ElsdcError::OpenCVError(e))?;

            // Copy data from ImageDouble to Mat
            for y in 0..ysize {
                for x in 0..xsize {
                    let value = *(*img_double).data.offset((y * xsize + x) as isize);
                    *mat.at_2d_mut::<f64>(y, x).map_err(|e| ElsdcError::OpenCVError(e))? = value;
                }
            }

            Ok(OpenCVImage { mat })
        }
    }
}

impl TryFrom<&str> for OpenCVImage {
    type Error = ElsdcError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        // 确保输入图像是 PGM 格式
        let pgm_path = ensure_pgm_image(path)?;

        // 读取 PGM 图像
        let mat = imgcodecs::imread(&pgm_path, imgcodecs::IMREAD_GRAYSCALE)
            .map_err(|e| ElsdcError::OpenCVError(e))?;

        // 将图像转换为 CV_64F 格式（双精度浮点数）
        let mut mat_64f = Mat::default();
        mat.convert_to(&mut mat_64f, opencv::core::CV_64F, 1.0, 0.0)
            .map_err(|e| ElsdcError::OpenCVError(e))?;

        Ok(OpenCVImage { mat: mat_64f })
    }
}

impl TryFrom<&String> for OpenCVImage {
    type Error = ElsdcError;

    fn try_from(path: &String) -> Result<Self, Self::Error> {
        OpenCVImage::try_from(path.as_str())
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
