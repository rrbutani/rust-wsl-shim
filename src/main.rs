// #[macro_use]
// extern crate text_io;

extern crate serde_json;

use serde_json::{Value, /*Error*/};

// use std::error::Error;
// use std::io::prelude::*;
use std::io::{stdin, Read};
use std::io::ErrorKind;
// use std::process::{Command, Stdio};

// const BUF_SIZE: usize = 1024 * 64;

fn main() {
	// let mut buf: [0u8; BUF_SIZE];

	// let mut header: String = String::new();
    let mut stdin = stdin();

// The LSP Spec (https://microsoft.github.io/language-server-protocol/specification)
// uses UTF-8.

	let mut header = String::new();

	loop {
		header.clear();

		match stdin.read_line(&mut header) {
			Ok(0) => {
				println!("Reached EOF. Good-bye!");
				return;
			},
			Ok(_) => {
				match parse_header(&header) {
					Some(bytes) => {
						match stdin.read_line(&mut header) {
							Ok(2) => {},
							_ => println!("Expecting \\r\\n after header as per LSP spec, got {}", header),
						}

						let mut buf = vec![0u8; bytes];
						stdin.read_exact(&mut buf[..]).unwrap();

						let v: Value = serde_json::from_slice(buf.as_slice()).unwrap();

						let ser = v.to_string();
						print!("Content-Length: {}\r\n\r\n", ser.len());
						print!("{}", ser);
					},
					None => println!("Invalid header: {}", header),
				}
			},
			Err(err) => match err.kind() {
				ErrorKind::InvalidData => println!("Data Error: {}", err),
				ErrorKind::UnexpectedEof => println!("Hit EOF: {}", err),
				_ => {
					println!("Unexpected error: {}", err);
				}
			},
		}
	}

    // loop {
    // 	match buf.read(&mut buf[..]) {
    // 		Some(0) => {
    // 			println!("Pipe closed. Good-bye!");
    // 			return;
    // 		},
    // 		Some(a) => {

    // 		},
    // 		Err(err) => {
    // 			println!("Something went wrong: {}", err);
    // 			return;
    // 		}
    // 	}
    // }

}

fn parse_header(header: &String) -> Option<usize> {
	let mut iter = header.split_whitespace();

	match (iter.next(), iter.next(), iter.next()) {
		(Some("Content-Length:"), Some(size), None) => {
			match size.parse() {
				Ok(size) => Some(size),
				Err(_) => None
			}
		},
		_ => None
	}


}
