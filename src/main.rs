use std::fs::File;
use std::fs::remove_file;
use std::io::Write;
use std::env;

fn main() {
    let temp_directory = env::temp_dir();
    let mut temp_file = temp_directory.join("temporary_file");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2  {
        // if 1 (extra) argument is provided, it is used as temp_file path
        // (can target other partitions as well)
        temp_file = std::path::PathBuf::from(&args[1]);
    }
    
    println!("freespace_wiper",);
    println!("Creating temporary file: {}", temp_file.display());
    
    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(&temp_file).unwrap();
    
    // Write to file until we get file i/o error
    let mut not_finished=true;
    while not_finished {
        
        if let Err(e) = file.write(b"11111111") {
            println!("File i/o-error. {} This is expected, and file probably filled whole hard disk and rewrite was succesfull.", e);
            println!("Deleting file...");
            if let Err(e) = remove_file(&temp_file) {
                println!("File deletion error. {} Clean file by hand.", e);
            } else {
                println!("File deletion succesfull.");
            }
            not_finished=false;
        }
    }
    println!("Program finished.");
}
