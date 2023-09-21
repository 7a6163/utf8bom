use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

fn add_utf8_bom(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut content = Vec::new();
    input_file.read_to_end(&mut content)?;

    let has_bom = content.starts_with(&UTF8_BOM);

    if has_bom {
        println!("The input file already has UTF-8 BOM.");
        return Ok(());
    }

    let output_file = File::create(output_path)?;
    let mut writer = output_file;

    // Write the UTF-8 BOM
    writer.write_all(&UTF8_BOM)?;

    // Write the remaining content
    writer.write_all(&content)?;

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
