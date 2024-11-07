use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure two file paths are provided
    if args.len() != 3 {
        eprintln!("Usage: {} <image1.tga> <image2.tga>", args[0]);
        std::process::exit(1);
    }

    let file_path1 = &args[1];
    let file_path2 = &args[2];

    // Read the files into byte vectors
    let image1 = read_file_to_bytes(file_path1)?;
    let image2 = read_file_to_bytes(file_path2)?;

    // Compare the lengths first
    if image1.len() != image2.len() {
        println!("The images are different: file sizes do not match.");
        std::process::exit(0);
    }

    // Compare the files byte by byte
    if image1 == image2 {
        println!("The images are identical.");
    } else {
        println!("The images are different: contents do not match.");
    }

    Ok(())
}

// Helper function to read a file into a byte vector
fn read_file_to_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
