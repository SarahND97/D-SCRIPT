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
    let interact: &String = &args[3];
    
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
            // Consumes the iterator, returns an (Optional) String
            let mut i = 0;
            let mut input: String = "".to_owned();
            for line in lines {
                if let Ok(ip) = line {
                    // println!("{}", ip);
                    if ip.starts_with('>') && i==0 {
                        let ip = rem_first_and_last(&ip);
                        input.push_str(ip);
                        input.push_str("\t");
                        // writeln!(to_write_file, "{}", ip.trim_start().to_string() + "\t").unwrap();
                    }
                    if ip.starts_with('>') && i==2 {
                        let ip = rem_first_and_last(&ip);
                        input.push_str(ip);
                    }
                }
                i = i + 1;
            }
            writeln!(to_write_file, "{}", input.to_string() + "\t" + interact).unwrap();
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

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    //chars.next_back();
    chars.as_str()
}