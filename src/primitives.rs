/**
 * File: /src/primitives.rs
 * Created Date: Wednesday, July 24th 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 9:18:34 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use libc::c_double;
use std::any::Any;

/// Represents a primitive shape that can be drawn on an image.
pub trait Primitive: Any {
    fn draw(&self, image: &mut dyn Image) -> Result<(), Box<dyn std::error::Error>>;
    fn to_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

/// Represents an image that can be drawn on.
pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_pixel(&mut self, x: u32, y: u32, value: f64) -> Result<(), Box<dyn std::error::Error>>;
    fn get_pixel(&self, x: u32, y: u32) -> Result<f64, Box<dyn std::error::Error>>;
    fn as_ptr(&self) -> *const c_double;
    fn as_mut_ptr(&mut self) -> *mut f64;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}