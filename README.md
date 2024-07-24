# Ellipse Detection Project

This project implements ellipse and circular arc detection in images using the ELSDc algorithm.

## Features

- Detect ellipses and circular arcs in grayscale images
- Visualize detected primitives
- Command-line interface for easy usage

## Installation

1. Ensure you have Rust and Cargo installed.
2. Clone this repository:
git clone https://github.com/yourusername/ellipse-detection.git
cd ellipse-detection
3. Build the project:
cargo build --release

## Usage

Run the program with:
cargo run --release -- [OPTIONS] <INPUT_IMAGE>

Options:
- `-o, --output <FILE>`: Specify output image file
- `-v, --verbose`: Enable verbose logging

For more details, run:
cargo run --release -- --help
