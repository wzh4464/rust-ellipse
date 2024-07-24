/**
 * File: /src/main.rs
 * Created Date: Monday, July 22nd 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 8:13:43 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use elsdc::pgm::ensure_pgm_image;
use elsdc::{
    elsdc::{detect_primitives, free_PImageDouble, free_outputs, read_pgm_image_double},
    ElsdcError,
};
use libc::{c_double, c_uint};
use opencv::core::{Mat, Scalar, Vector, CV_8UC3};
use opencv::imgcodecs;
use opencv::prelude::*;
use std::ffi::CString;
use std::ptr;
use clap::Parser;
use log::info;
use env_logger::Env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input image file
    #[clap(value_parser)]
    input: String,

    /// Output image file
    #[clap(short, long, value_parser)]
    output: Option<String>,

    /// Verbose mode
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), ElsdcError> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    if args.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    info!("Processing image: {}", args.input);

    let filename =  args.input;
    let pgm_filename = ensure_pgm_image(&filename)?;

    let cstring_filename = CString::new(pgm_filename.clone())
        .map_err(|e| ElsdcError::DetectionError(e.to_string()))?;
    let img_double = unsafe { read_pgm_image_double(cstring_filename.as_ptr()) };

    if img_double.is_null() {
        return Err(ElsdcError::ImageReadError(
            "Failed to read PGM image".into(),
        ));
    }

    let xsize = unsafe { (*img_double).xsize };
    let ysize = unsafe { (*img_double).ysize };

    let mut ell_out: *mut elsdc::ring::Ring = ptr::null_mut();
    let mut ell_labels: *mut libc::c_int = ptr::null_mut();
    let mut ell_count: libc::c_int = 0;
    let mut out: *mut libc::c_int = ptr::null_mut();

    unsafe {
        let result = detect_primitives(
            &mut ell_out,
            &mut ell_labels,
            &mut ell_count,
            &mut out,
            (*img_double).data as *mut c_double,
            xsize,
            ysize,
        );

        if result == 0 {
            info!("Detection successful!");

            let mut img_all_ellipses = Mat::new_rows_cols_with_default(
                ysize as i32,
                xsize as i32,
                CV_8UC3,
                Scalar::all(0.0),
            )
            .map_err(|e| ElsdcError::OpenCVError(e))?;

            // Copy the image data to the Mat
            let img_data = img_all_ellipses.data_mut();
            for i in 0..(xsize * ysize) as usize {
                let pixel_value = (*img_double).data.add(i).read() as u8;
                let offset = i * 3;
                *img_data.offset(offset as isize) = pixel_value;
                *img_data.offset(offset as isize + 1) = pixel_value;
                *img_data.offset(offset as isize + 2) = pixel_value;
            }

            for i in 0..ell_count as c_uint {
                let ring = ell_out.add(i as usize).read();
                ring.draw(&mut img_all_ellipses)?;
            }

            let output_path_all = "result/output_all_rings.png";
            let params = Vector::new();
            imgcodecs::imwrite(&output_path_all, &img_all_ellipses, &params)?;
            info!("Saved detected rings image to {}", output_path_all);
        } else {
            return Err(ElsdcError::DetectionError("Detection failed!".into()));
        }

        free_outputs(ell_out, ell_labels, ell_count, out);
        free_PImageDouble(img_double);
    }

    Ok(())
}
