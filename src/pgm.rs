/**
 * File: /src/pgm.rs
 * Created Date: Monday, July 22nd 2024
 * Author: Zihan
 * -----
 * Last Modified: Wednesday, 24th July 2024 7:21:38 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use opencv::core::Vector;
use opencv::imgcodecs::{self, IMREAD_GRAYSCALE};

use crate::ElsdcError;

#[derive(Debug)]
pub struct PImageDouble {
    pub xsize: usize,
    pub ysize: usize,
    pub data: Vec<f64>,
}

pub fn read_pgm_image_double_rust(filename: &str) -> io::Result<PImageDouble> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut magic_number = String::new();
    reader.read_line(&mut magic_number)?;
    if !magic_number.starts_with("P2") && !magic_number.starts_with("P5") {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a PGM file"));
    }
    let bin = magic_number.starts_with("P5");

    let (xsize, ysize, depth) = read_pgm_header(&mut reader)?;

    if depth == 0 {
        eprintln!("Warning: depth=0, probably invalid PGM file.");
    }

    let mut data = vec![0.0; xsize * ysize];

    if bin {
        let mut buffer = vec![0u8; xsize * ysize];
        reader.read_exact(&mut buffer)?;
        for (i, &value) in buffer.iter().enumerate() {
            data[i] = value as f64;
        }
    } else {
        for value in &mut data {
            let mut pixel_value = String::new();
            reader.read_line(&mut pixel_value)?;
            *value = pixel_value.trim().parse::<f64>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid pixel value: {}", e),
                )
            })?;
        }
    }

    Ok(PImageDouble { xsize, ysize, data })
}

pub fn read_pgm_header<R: BufRead>(reader: &mut R) -> io::Result<(usize, usize, usize)> {
    let mut width = String::new();
    let mut height = String::new();
    let mut depth = String::new();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        if width.is_empty() {
            if let Some(part) = parts.next() {
                width.push_str(part);
            }
        }
        if height.is_empty() {
            if let Some(part) = parts.next() {
                height.push_str(part);
            }
        }
        if depth.is_empty() {
            if let Some(part) = parts.next() {
                depth.push_str(part);
            }
        }
        if !width.is_empty() && !height.is_empty() && !depth.is_empty() {
            break;
        }
    }

    let width = width
        .trim()
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid width: {}", e)))?;
    let height = height.trim().parse::<usize>().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid height: {}", e))
    })?;
    let depth = depth
        .trim()
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid depth: {}", e)))?;

    Ok((width, height, depth))
}

pub fn scale_data(data: &mut [f64], max_value: f64) {
    let min = *data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    for v in data.iter_mut() {
        *v = (*v - min) / (max - min) * max_value;
    }
}

pub fn ensure_pgm_image(filename: &str) -> Result<String, ElsdcError> {
    if filename.to_lowercase().ends_with(".pgm") {
        return Ok(filename.to_string());
    }

    let img =
        imgcodecs::imread(filename, IMREAD_GRAYSCALE).map_err(|e| ElsdcError::OpenCVError(e))?;

    let new_filename = format!(
        "pgm/{}.pgm",
        Path::new(filename)
            .file_stem()
            .ok_or_else(|| ElsdcError::DetectionError("Invalid filename".to_string()))?
            .to_string_lossy()
    );

    fs::create_dir_all("pgm")?;

    imgcodecs::imwrite(&new_filename, &img, &Vector::new())
        .map_err(|e| ElsdcError::OpenCVError(e))?;

    Ok(new_filename)
}
