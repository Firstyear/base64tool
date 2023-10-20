use std::fs::File;
use std::io::Seek;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

/// Encode a binary file into base64
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The path to the input binary file.
    input: PathBuf,
    /// The path to the output base64 file.
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.output.exists() {
        eprintln!("Refusing to proceed, output file already exists.");
        return;
    }

    let mut file_input = match File::open(&args.input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open path - {:?}", e);
            return;
        }
    };

    let mut file_output = match File::create(&args.output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open path - {:?}", e);
            return;
        }
    };

    if let Err(e) = file_input.seek(std::io::SeekFrom::Start(0)) {
        eprintln!("Unable to seek input -> {:?}", e);
        return;
    };

    if let Err(e) = file_output.seek(std::io::SeekFrom::Start(0)) {
        eprintln!("Unable to seek output -> {:?}", e);
        return;
    };

    let mut enc =
        base64::write::EncoderWriter::new(file_output, &base64::engine::general_purpose::STANDARD);

    // handle errors as you normally would
    if let Err(e) = std::io::copy(&mut file_input, &mut enc) {
        eprintln!("Failed to copy from input to base64 encoder -> {:?}", e);
        return;
    }

    match enc.finish() {
        Ok(mut file_output) => {
            if let Err(e) = file_output.flush() {
                eprintln!("Failed to flush bytes to output -> {:?}", e);
            } else {
                println!("Ok");
            }
        }
        Err(e) => {
            eprintln!("Failed to finalise base64 encoder -> {:?}", e);
        }
    }
}
