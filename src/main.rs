use std::fs::File;
use std::io::Write;
use std::env;

fn main() {
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("file");
    
    println!("freespace_wiper",);
    println!("Creating temporary file: {}", temp_file.display());

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(temp_file).unwrap();

    // Write a &str in the file (ignoring the result).
    let mut n = 0;
    while n < 10 {
        writeln!(&mut file, "Hello World!").unwrap();
        n += 1;
    }

}
