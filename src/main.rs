/**
 * File: /src/main.rs
 * Created Date: Monday, July 22nd 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 25th July 2024 12:38:42 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

use elsdc::{ElsdcError, OpenCVImage};
use env_logger::Env;
use log::info;
use clap::Parser;
use elsdc::save_matrix_to_file;

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
    
    let (primitives, compatibility_matrix) = elsdc::elsdc::detect_primitives_on_real_image(&args.input)?;
    info!(
        "Detection successful! Found {} primitives",
        primitives.len()
    );

    // Load the image again for drawing (since process_image doesn't return the image)
    let mut image = OpenCVImage::try_from(&args.input)?;

    // Draw primitives
    for primitive in &primitives {
        primitive.draw(&mut image)?;
    }

    // Save result
    let output_path = args.output.as_ref().map(|s| s.as_str()).unwrap_or("result/output_all_rings.png");
    image.save(output_path)?;
    info!("Saved detected rings image to {}", output_path);
    
    let matrix_output = args.output.as_ref()
        .map(|s| s.replace(".png", "_matrix.txt"))
        .unwrap_or_else(|| "result/compatibility_matrix.txt".to_string());
    save_matrix_to_file(&compatibility_matrix, &matrix_output)?;
    info!("Saved compatibility matrix to {}", matrix_output);
    
    Ok(())
}
