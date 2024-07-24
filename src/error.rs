/**
 * File: /src/error.rs
 * Created Date: Wednesday, July 24th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 9:18:39 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use std::fmt;
use std::error::Error;
use opencv::Error as OpenCVError;

#[derive(Debug)]
pub enum ElsdcError {
    IoError(std::io::Error),
    OpenCVError(OpenCVError),
    ImageReadError(String),
    DetectionError(String),
    ImageConversionError,
    // 添加其他可能的错误类型
}

impl fmt::Display for ElsdcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ElsdcError::IoError(e) => write!(f, "IO error: {}", e),
            ElsdcError::OpenCVError(e) => write!(f, "OpenCV error: {}", e),
            ElsdcError::ImageReadError(s) => write!(f, "Image read error: {}", s),
            ElsdcError::DetectionError(s) => write!(f, "Detection error: {}", s),
            ElsdcError::ImageConversionError => write!(f, "Image conversion error"),
        }
    }
}

impl Error for ElsdcError {}

impl From<std::io::Error> for ElsdcError {
    fn from(error: std::io::Error) -> Self {
        ElsdcError::IoError(error)
    }
}

impl From<OpenCVError> for ElsdcError {
    fn from(error: OpenCVError) -> Self {
        ElsdcError::OpenCVError(error)
    }
}

impl From<Box<dyn std::error::Error>> for ElsdcError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ElsdcError::DetectionError(error.to_string())
    }
}
