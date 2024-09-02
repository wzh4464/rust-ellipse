/**
 * File: /src/main.rs
 * Created Date: Monday, July 22nd 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 8th August 2024 8:48:14 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use elsdc::{ElsdcError, OpenCVImage};
use env_logger::Env;
use log::{info, error};
use clap::Parser;
use elsdc::save_matrix_to_file;
use elsdc::ring::Ring;

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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    log::set_max_level(if args.verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info });

    info!("Processing image: {}", args.input);

    let (primitives, compatibility_matrix) = match elsdc::elsdc::detect_primitives_on_real_image(&args.input) {
        Ok(result) => result,
        Err(e) => {
            error!("Failed to detect primitives: {:?}", e);
            return Err(e);
        }
    };

    info!(
        "Detection successful! Found {} primitives",
        primitives.len()
    );

    // 计算和输出IoU矩阵
    let rings: Vec<&Ring> = primitives.iter().filter_map(|p| p.as_any().downcast_ref::<Ring>()).collect();
    if rings.is_empty() {
        error!("No rings detected.");
        return Ok(());
    }

    let iou_matrix = Ring::generate_compatibility_matrix(&rings.iter().map(|&r| *r).collect::<Vec<Ring>>());

    println!("IoU Matrix:");
    for row in &iou_matrix {
        for value in row {
            print!("{:.2} ", value);
        }
        println!();
    }

    // Load the image again for drawing (since process_image doesn't return the image)
    let mut image = match OpenCVImage::try_from(&args.input) {
        Ok(img) => {
            img
        }
        Err(e) => {
            error!("Failed to load image: {:?}", e);
            return Err(e);
        }
    };

    // Draw primitives
    for primitive in &primitives {
        if let Err(e) = primitive.draw(&mut image) {
            error!("Failed to draw primitive: {:?}", e);
        }
    }
    // debug log, primitive numbers and details
    for (i, primitive) in primitives.iter().enumerate() {
        info!("Primitive {}: {}", i, primitive.to_string());
    }

    // Save result
    let output_path = args.output.as_ref().map(|s| s.as_str()).unwrap_or("result/output_all_rings.png");
    if let Err(e) = image.save(output_path) {
        error!("Failed to save image: {:?}", e);
    } else {
        info!("Saved detected rings image to {}", output_path);
    }

    let matrix_output = args.output.as_ref()
        .map(|s| s.replace(".png", "_matrix.txt"))
        .unwrap_or_else(|| "result/compatibility_matrix.txt".to_string());
    if let Err(e) = save_matrix_to_file(&compatibility_matrix, &matrix_output) {
        error!("Failed to save compatibility matrix: {:?}", e);
    } else {
        info!("Saved compatibility matrix to {}", matrix_output);
    }

    Ok(())
}
