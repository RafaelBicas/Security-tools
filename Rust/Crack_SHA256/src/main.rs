#![allow(unused)]

use std::env;
use std::fs::File; //to get files
use std::io::{BufRead, BufReader};
use digest::block_buffer::Error;
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Invalid argument");
		println!("Example: cargo run <sha256 hash>");
		exit(1);
	}

	let wanted_hash: &String = &args[1];
	let wordlist_file: &str = "src/rockyou.txt";
	let mut attempts: i32 = 1;
	println!("Attempting to hack: {}\n", wanted_hash);
	let password_list: Result<File, Error> = File::open(wordlist_file).unwrap();
	let reader: BufReader<File> = BufReader::new(password_list); //if i need to read one file over and over again, it will improve the speed of it
	
	for line: Result<String, Error> in reader.line() {
		let line: String = line.unwrap();
		let password: String = line.trim().to_owned().into_bytes();
		let password_hash: String = format!("{:x}", sha256::digest(&password));
		println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password, password_hash);

		if &password_hash == wanted_hash {
			println!("Password hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password, password_hash);
			exit(0);
		}
		attempts += 1;
	}
println!("Password hash not found!");
}
