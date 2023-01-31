use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

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

    for entry in fs::read_dir(dir_).unwrap() {
        let contents = fs::read_to_string(entry.unwrap().path())
        .expect("Should have been able to read the file");
        writeln!(to_write_file, "{}", contents.trim()).unwrap();
    }
}






