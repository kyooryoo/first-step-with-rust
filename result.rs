use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();
    // access the file
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };
    // access the file content
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    Ok(string)
}

fn main() {
    let content = read_file_contents(PathBuf::from("result.rs"));
    if content.is_ok() {
        println!("The program found the main file.");
        println!("The content is as follows\n{:?}", content.unwrap());
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("\nThe file that doesn't exist.");
    }
}
