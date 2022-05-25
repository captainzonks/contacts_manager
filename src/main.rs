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
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    id: i32,
    name: String,
    email: String,
}

fn read_from_csv(contents: String) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut contents = vec![];
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => contents.push(record),
            Err(_) => {}
        }
    }
    Ok(contents)
}

fn print_contacts(record: Record) {
    println!("{:?}", record);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/p2_data.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let records = read_from_csv(contents);
    match records {
        Ok(records) => {
            for entry in records
            {
                print_contacts(entry);
            }
        },
        Err(e) => {
            println!("error running: {}", e);
            process::exit(1);
        }
    }
    Ok(())
}
