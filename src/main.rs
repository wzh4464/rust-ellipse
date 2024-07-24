use clap::Parser;
use elsdc::elsdc::detect_primitives_on_real_image;
/**
 * File: /src/main.rs
 * Created Date: Monday, July 22nd 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 25th July 2024 12:16:49 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/
use elsdc::{ElsdcError, OpenCVImage};
use env_logger::Env;
use log::info;

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
    
    let primitives = detect_primitives_on_real_image(&args.input)?;
    info!("Detection successful! Found {} primitives", primitives.len());

    // Load the image again for drawing (since process_image doesn't return the image)
    let mut image = OpenCVImage::try_from(&args.input)?;

    // Draw primitives
    for primitive in &primitives {
        primitive.draw(&mut image)?;
    }

    // Save result
    let output_path = args.output.unwrap_or_else(|| "result/output_all_rings.png".to_string());
    image.save(&output_path)?;
    info!("Saved detected rings image to {}", output_path);

    Ok(())
}
