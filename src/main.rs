use clap::{Arg, Command};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Write};

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open input file: {}", e)))?;
    
    let output_file = File::create(output_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to create output file: {}", e)))?;
    
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    io::copy(&mut &input_file, &mut encoder)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to compress file: {}", e)))?;

    encoder.finish()?;
    Ok(())
}

fn main() {
    let matches = Command::new("File Compressor")
        .version("1.0")
        .author("Pulkit Garg <business.pulkitgarg@gmail.com>")
        .about("Compresses files using Gzip")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("The input file to compress")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("The output compressed file")
                .required(true),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();

    match compress_file(input_path, output_path) {
        Ok(_) => println!("File '{}' compressed successfully to '{}'.", input_path, output_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
