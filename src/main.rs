use std::{ fs::File, io::{ self, BufRead, BufReader } };

mod unpin_by_hash;
use colored::Colorize;

fn main() -> io::Result<()> {
    let mut hashes: Vec<String> = Vec::new();

    println!();
    println!("{:^10}", "Unpin By File:".blue());
    println!("{:^20}", "Enter the file path (q to quit):");

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;

    let file_path = file_path.trim();

    if file_path != "q" {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);

        let lines: Vec<_> = buf_reader.lines().collect(); // Collect lines into a Vec

        println!();

        for line in lines.into_iter() {
            // Convert Vec into an iterator
            if let Ok(line) = line {
                let columns: Vec<&str> = line.split(',').collect();
                if let Some(first_column) = columns.get(0) {
                    hashes.push(first_column.to_string());
                }
            }
        }

        for (index, hash) in hashes.iter().skip(1).enumerate() {
            let hash_without_quotes = hash.trim_matches('"');

            if let Err(_error) = unpin_by_hash::main_with_args(hash_without_quotes, index + 1) {
            }
        }

        println!();
        println!("{}", "Program finished".bold().green());
    }

    Ok(())
}
