use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::num::ParseIntError;
use std::path::Path;

fn main() {}


#[allow(unused)]
fn double_arg(mut argv: env::Args) -> Result<i32, String> {
	argv.nth(1).
		ok_or("please give at least one argument".to_string()).
		and_then(|arg| arg.parse::<i32>().map_err(
			|err| err.to_string()
		)).
		map(|n| 2 * n)
}

#[allow(unused)]
fn custom_error(val: i32) -> Result<bool, String> {
	if val <= 0 {
		Ok(true)
	} else {
		Err("".to_string())
	}
}

#[allow(unused)]
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
	let mut file = File::open(file_path).map_err(|e| e.to_string())?;
	let mut contents = String::new();
	file.
		read_to_string(&mut contents).
		map_err(|err| err.to_string())?;
	let n = contents.trim().parse::<i32>().map_err(|err| err.to_string())?;

	Ok(2 * n)
}

#[derive(Debug)]
#[allow(unused)]
enum MyError {
	IO(Error),
	Parse(ParseIntError),
}

impl From<Error> for MyError {
	fn from(error: Error) -> Self {
		MyError::IO(error)
	}
}

impl From<ParseIntError> for MyError {
	fn from(error: ParseIntError) -> Self {
		MyError::Parse(error)
	}
}

fn double_file<P: AsRef<Path>>(file_path: P) -> Result<i32, MyError> {
	let mut file = File::open(file_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let n = contents.trim().parse::<i32>()?;

	Ok(2 * n)
}

#[test]
fn test_file() {
	match double_file("foobar") {
		Ok(val) => println!("{}", val),
		Err(err) => println!("Error: {:?}", err),
	}
}