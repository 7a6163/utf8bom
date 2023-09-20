use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::process;

fn add_utf8_bom(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // Read the first 3 bytes
    let mut bom = [0u8; 3];
    let _ = reader.read_exact(&mut bom);

    if bom != [0xEF, 0xBB, 0xBF] {
        // Not UTF-8 BOM present in the input file
        print!("Input file does not have UTF-8 BOM.");
        return Ok(())
    }

    // Write the UTF-8 BOM
    writer.write_all(&[0xEF, 0xBB, 0xBF])?;

    // Write the remaining content
    io::copy(&mut reader, &mut writer)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <input file> <output file>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    if let Err(err) = add_utf8_bom(input_path, output_path) {
        eprintln!("An error occurred: {}", err);
        process::exit(1);
    }
}
