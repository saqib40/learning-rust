use std::fs::File;
use std::io::Error;

fn open_file() -> Result<File, Error> {
    File::open("file.txt")
}

pub fn recover() {
    match open_file() {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Failed to open the file: {}", e),
    }
    println!("This line will be executed irrespectively");
}
