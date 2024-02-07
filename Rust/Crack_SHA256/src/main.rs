#![allow(unused)]

use std::env;
use std::path::Path;
use std::fs;
use rand::distributions::DistString;
use rand::{distributions::Alphanumeric, Rng};
use std::process::exit;
use std::fs::File;
use std::io::Write; //to write the file
use std::io::prelude::*; //Can be used instead of the write one 
use std::fs::OpenOptions; // to allow to append data
use std::io::{BufRead, BufReader};
//use core::fmt::Error;
use std::io::Error;
use digest::block_buffer::Error;

const SHORT_URL_SIZE: u8 = 8; 
const FILE_PATH: &str = "/home/ellie/Documents/first_project/src/test.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid Input");
        println!("Example: cargo run https://google.com");
        exit(1);
    }
    println!("{}", args[1]);
    generate_short_url();
    //create_file(file);
    let a = file_exists(FILE_PATH.to_string());
    println!("{}", a);
    create_file(FILE_PATH);
    update_file("let it go", "Hell")
    
}


fn generate_short_url() -> String {
    let short_url: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SHORT_URL_SIZE.try_into().unwrap())
        .map(char::from)
        .collect();
    println!("{}", short_url);
    short_url
}

fn create_file(path_file: &str) {
    let mut file = File::create(path_file).expect("File failed to create");
}

fn update_file(full_url: &str, short_url: &str) {
    let data_update: String = format!("{},{}\n", full_url, short_url);
    let data_update_slice: &str = &*data_update;
    let mut mapping_file = OpenOptions::new().write(true).append(true).open(FILE_PATH).expect("unable to open the file");
    mapping_file.write_all(data_update_slice.as_bytes()).expect("Failed to write in the file");
}

fn redirect_url(url: &str) {
    let mapping_file = match File::open(FILE_PATH){
        Ok(file) => file,
        Err(_) => {
            println!("Error opening mapping file");
            return;
        }
    };
    let reader: BufReader<File> = BufReader::new(mapping_file);
	//agr vou fazer com q procure strings `procurar at[e virgula ou depois da virgula`
	for line in reader.lines() {
		let line: String = line.unwrap();
		
    }
}


fn file_exists(file_path: String) -> bool {
    let file_existance = Path::new(&file_path).exists();
    file_existance
}