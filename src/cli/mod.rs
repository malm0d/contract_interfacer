use clap::{value_parser, Parser, Subcommand};
use std::panic;
use crate::file::{write_to_csv, read_from_csv, Record};

pub mod args;
pub mod command;
pub mod validate;

//Here, read from the csv and deal with the derivation numbers
//then run the execute function from command.
// pub fn get_derivation_numbers_from_file(file_path: &str) -> Result<Vec<u32>, Box<dyn Error>> {
//     let records = read_from_csv(file_path)?;
//     let derivation_numbers = records.iter().map(|record| record.derivation).collect();
//     Ok(derivation_numbers)
// }

pub fn run() {}