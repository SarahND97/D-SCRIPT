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
    let ourdir_: &String = &args[2];
    // this allows to open a file in file append mode to write (or create it)
    // let mut to_write_file = BufWriter::new(
    //     OpenOptions::new()
    //     .append(true)
    //     .create(true) // Optionally create
    //     .open(outfile)
    //     .expect("Unable to open file"),
    // );
    
    // Input is a directory and we want to read every -fasta-file in the directory
    for entry in fs::read_dir(dir_).unwrap() {
        // let mut outfile: String = args[2].to_owned();
        if let Ok(lines) = read_lines(entry.unwrap().path()) {
            // Consumes the iterator, returns an (Optional) String
            let mut i = 0;
            let mut title: String = "".to_owned();
            let mut input: String = "".to_owned();
            for line in lines {
                if let Ok(ip) = line {
                    // println!("{}", ip);
                    // if ip.starts_with('>') && i==0 {
                    //     let ip_title = rem_first(&ip);
                    //     title.push_str(">");
                    //     title.push_str(&ip_title);
                    //     title.push_str("_");
                    //     title.push_str(&ip_title);
                    //     title.push_str("\n");
                    //     // input.push_str("\n");   
                    //     // writeln!(to_write_file, "{}", input).unwrap();
                    // }
                    // else {
                        // if i==1 {
                        //     input.push_str(&ip);
                        //     input.push_str(":");
                        //     input.push_str(&ip);
                        // }
                    if ip.starts_with('>') && i==2 {
                        let ip_title = rem_first(&ip);
                        title.push_str(">");
                        title.push_str(&ip_title);
                        title.push_str("_");
                        title.push_str(&ip_title);
                        title.push_str("\n");
                        // input.push_str("\n");   
                        // writeln!(to_write_file, "{}", input).unwrap();
                    }
                    if i==3 {
                    //     input.push_str(&ip);
                    //     //writeln!(to_write_file, "{}", input).unwrap()
                        input.push_str(&ip);
                        input.push_str(":");
                        input.push_str(&ip);
                    }
                //}
                    // if ip.starts_with('>') && i==2 {
                    //     let ip = rem_first_and_last(&ip);
                    //     input.push_str(ip);
                    // }
                }
                i = i + 1;
            }
            let outfile = rem_first_and_last(&title);
            //outfile.to_string().push_str(".fasta");
            //println!("{}",ourdir_.to_owned()+&outfile+"B"+".fasta");
            let mut to_write_file = BufWriter::new(
                OpenOptions::new()
                .append(true)
                .create(true) // Optionally create
                .open(ourdir_.to_owned()+&outfile+".fasta")
                .expect("Unable to open file"),
            );
            title.push_str(&input);
            writeln!(to_write_file, "{}", title).unwrap();
            //writeln!(to_write_file, "{}", input.to_string() + "\t" + "1.0").unwrap();
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
    chars.next_back();
    // chars.next_back();
    chars.as_str()
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    // chars.next_back();
    // chars.next_back();
    chars.as_str()
}