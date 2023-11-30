use std::{io::{Write, BufReader, Read, BufWriter}, num::IntErrorKind};

fn get_input(query: &str) -> String {
    print!("{query}");
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}

fn process_file_data(data: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut processed_data = Vec::with_capacity(data.len());

    for byte in data {
        processed_data.push(byte ^ key);
    }

    processed_data
}

fn main() {
    loop {
        println!("------------------");

        let input_file_name = get_input("Enter file name to process: ");
        let input_file = match std::fs::File::open(&input_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't open file {}, {}", input_file_name, err);
                continue
            }
        };

        let key = match get_input("Enter a key for file encryption/decryption: ")
        .parse::<u8>() {
            Ok(key) => key,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Key mustn't be emty"),
                    IntErrorKind::InvalidDigit => println!("Enter correct number"),
                    IntErrorKind::PosOverflow => println!("Number must be in range (0: 255]"),
                    _ => println!("Error gettin key")
                }
                continue
            }
        };

        if key == 0 {
            println!("0 is useless key");
            continue
        }

        let mut reader = BufReader::new(input_file);
        let mut input_data = Vec::new();

        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Error reading file: {err}");
            continue
        }

        let processed_data = process_file_data(&input_data, key);

        let output_file_name = get_input("Enter file name to output: ");
        let output_file = match std::fs::File::create(&output_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't create file {}, {}", output_file_name, err);
                continue
            }
        };

        let mut writer = BufWriter::new(output_file);

        if let Err(err) = writer.write_all(&processed_data) {
            println!("Error writig to output file: {err}");
            continue
        }

        println!();
    }
}
