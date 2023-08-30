use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input_file> <output_directory> <reads_per_file>",
            args[0]
        );
        std::process::exit(1);
    }

    let input_filename = &args[1];
    let output_dir = &args[2];
    let reads_per_file: usize = args[3].parse()?;

    fs::create_dir_all(output_dir)?;

    let input_file = File::open(input_filename)?;
    let reader = BufReader::new(input_file);

    let mut output_count = 0;
    let mut read_count = 0;
    let mut output_file: Option<File> = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') || line.starts_with('@') {
            read_count += 1;
            if read_count > reads_per_file {
                read_count = 1;
                output_count += 1;

                if let Some(mut file) = output_file.take() {
                    file.flush()?;
                }

                let output_filename = format!("output_{}.fasta", output_count);
                let output_path = Path::new(output_dir).join(&output_filename);
                output_file = Some(File::create(output_path)?);
            }
        }

        if let Some(ref mut file) = output_file {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}
