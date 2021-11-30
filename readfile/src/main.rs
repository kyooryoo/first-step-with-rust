// this program is for demo error handling with Result
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file(path: PathBuf) -> Result<String, Error> {
    // create a variable for saving the file content
    let mut string = String::new();

    // try opening file with specified path
    let mut file: File = match File::open(path) {
        // if path opened well, assign file handler
        Ok(file_handle) => file_handle,
        // if path cannot open, return to main with Err
        Err(io_error) => return Err(io_error),
    };

    // try reading file content to string
    match file.read_to_string(&mut string) {
        // if file content read well, pass
        Ok(_) => (),
        // if file content cannot read, return to main
        Err(io_error) => return Err(io_error),
    };

    // return Ok with value to main
    Ok(string)
}

fn main() {
    println!("Hello, readfile!");

    let exist = read_file(PathBuf::from("src/main.rs"));
    if exist.is_ok() {
        println!("\nThe program found the main file.");
        println!("Content of the main.rs:\n{}", exist.unwrap());
    }

    let noexist = read_file(PathBuf::from("nosuchfile"));
    if noexist.is_err() {
        println!("\nCannot find your specified file!");
        println!("Detail of the error:\n{:?}", noexist.unwrap_err());
    }
}
