// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::error::Error;
use std::{fs, io, process};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    id: u16,
    name: String,
    email: String,
}

fn read_from_csv(contents: &String) -> (Result<Vec<Record>, Box<dyn Error>>, u16) {
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut contents = vec![];
    let mut count = 0;
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                contents.push(record);
                count += 1;
            },
            Err(e) => {}
        }
    }
    (Ok(contents), count)
}

fn write_to_csv(to_write: Record, file: &mut File) -> Result<(), Box<dyn Error>> {
    let string = to_write.id.to_string() + "," + to_write.name.as_str() + "," + to_write.email.as_str() + "\n";

    file.write_all(string.as_bytes()).expect("failed to write to file");

    Ok(())
}

fn print_contacts(record: &Record) {
    println!("{:?}", record);
}

fn display_menu() {
    println!("1) Show contacts");
    println!("2) Add contact");
    println!("3) Exit")
}

fn get_csv_input() -> (String, String) {
    println!("Please enter a name:");
    let name = get_string_input();
    println!("Please enter an email:");
    let email = get_string_input();

    (name.to_owned(), email.to_owned())
}

fn create_csv_record(id: u16, name: &str, email: &str) -> Record {
    let entry = Record {
        id,
        name: name.to_owned(),
        email: email.to_owned(),
    };
    entry
}

fn get_string_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("error with input: {}", e);
            process::exit(1);
        }
    }

    let choice = buffer.trim();
    choice.to_owned()
}

fn get_integer_input() -> i32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("error with input: {}", e);
            process::exit(1);
        }
    }

    let choice = buffer.trim().parse::<i32>();
    match choice {
        Ok(i) => return i,
        Err(e) => {
            println!("error with input: {}", e);
            process::exit(1);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut quit = false;
    while !quit {

        let file_path = "src/p2_data.csv";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(file_path);

        let mut contents = String::new();
        let records;
        match &mut file {
            Ok(file) => {
                file.read_to_string(&mut contents)?;
                records = read_from_csv(&contents);
                println!("records length: {:?}", &records.1);
            }
            Err(e) => {
                println!("file error: {:?}", e);
                process::exit(1);
            }
        }

        display_menu();
        match get_integer_input() {
            1 => {
                match &records.0 {
                    Ok(records) => {
                        for entry in records
                        {
                            print_contacts(entry);
                        }
                    }
                    Err(e) => {
                        println!("error running: {}", e);
                        process::exit(1);
                    }
                }
            }
            2 => {
                let mut file_ref = file.as_mut().unwrap();
                let strings = get_csv_input();
                match &records.0 {
                    Ok(records) => {
                        let new_id = records.len();
                        let new_record = create_csv_record(new_id as u16, strings.0.as_str(), strings.1.as_str());
                        match write_to_csv(new_record, file_ref) {
                            Ok(_) => {
                                println!("Successfully wrote to csv");
                            }
                            Err(e) => {
                                println!("error writing to file: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        println!("error running: {}", e);
                        process::exit(1);
                    }
                }
            }
            _ => {
                quit = true;
            }
        }
    }

    Ok(())
}
