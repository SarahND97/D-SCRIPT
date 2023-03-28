use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write, BufRead};
use std::fs::File;
use std::path::Path;

fn main(){    
    // gather cmd arguments
    let args: Vec<String> = env::args().collect();
    let dir_: &String = &args[1];
    let outfile: &String = &args[2];
    
    // this allows to open a file in file append mode to write (or create it)
    let mut to_write_file = BufWriter::new(
        OpenOptions::new()
        .append(true)
        .create(true) // Optionally create
        .open(outfile)
        .expect("Unable to open file"),
    );
    
    // Input is a directory and we want to read every -fasta-file in the directory
    for entry in fs::read_dir(dir_).unwrap() {

        if let Ok(lines) = read_lines(entry.unwrap().path()) {
            for line in lines {
                if let Ok(ip) = line {
                    writeln!(to_write_file, "{}", ip.to_string()).unwrap();
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}