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
    let shuffle_file: &String = &args[2];
    // this allows to open a file in file append mode to write (or create it)
    // let mut to_write_file = BufWriter::new(
    //     OpenOptions::new()
    //     .append(true)
    //     .create(true) // Optionally create
    //     .open(outfile)
    //     .expect("Unable to open file"),
    // );

    // First read shuffled file
    if let Ok(shuffled_lines) = read_lines(shuffle_file)
    {
        for shuffled_line in shuffled_lines {
            let shuffled_line = shuffled_line.unwrap();
            let mut parts = shuffled_line.split_whitespace();
            parts.next_back();
            // let mut line_number = 0;
            let mut input: String = "".to_owned();
            for part in parts {
                // println!("{}",part);
                // your_path.as_path().display().to_string()
                let mut temp_part = rem_two_last(&part);
                for mut entry in fs::read_dir(dir_).unwrap() {
                    // .as_ref().unwrap()
                    // println!("{:?}",entry.unwrap().path().file_stem());//.display().to_string());
                    // println!("{}",temp_part);
                    // reference.map(str::to_string)
                    // entry.unwrap().path().file_stem().as_str()
                    if  entry.unwrap().path().file_stem().unwrap().to_str() == Some(temp_part) {
                        // part includes the chain
                        // temp_part only includes protein-name
                        // println!("{}",line_number);
                        // println!("{}",shuffled_line);
                        // println!("{}",part);
                        // TODO: find the correct fasta_files and the correct chains 
                        // merge them together in a new fasta_file that can be analyzes by omegafold
                        if let Ok(fasta_file_contents) = read_lines(dir_.to_owned()+temp_part+".fasta") {
                        // println!("{:?}",fasta_file_contents.nth(1));
                            // let first_element = fasta_file_contents.nth(0).as_ref().map(<String, std::io::Error>::as_str).unwrap();
                            println!("{}",fasta_file_contents.nth(1).unwrap().to_string());
                            // for content in fasta_file_contents {
                            //     if content.starts_with('>') {
                            //         if rem_first(content)==part {
                            //             input.push_str(">");
                            //             input.push_str(&part);
                            //             input.push_str("\n"); 
                            //             // input.push_str(fasta_file_contents.nth(0));
                            //             input.push_str("\n");       
                            //         }
                            //     }
                            // }
                        }
                        // 
                    }
                }
                // line_number = line_number + 1;    
            }
            
        }
    }
    //
    // list.contains(&x)s
    // let parts = .split(" ");

    // This gives an iterator, which you can loop over, or collect() into a vector. For example:



    // TODO: get the pairs from the file 
    // get the two correct fasta-files
    // get the correct chains from the fasta-files
    // merge them together in a new file 

    
    // Input is a directory and we want to read every -fasta-file in the directory
    // for entry in fs::read_dir(dir_).unwrap() {
    //     let mut outfile: String = args[2].to_owned();
    //     if let Ok(lines) = read_lines(entry.unwrap().path()) {
    //         // Consumes the iterator, returns an (Optional) String
    //         let mut i = 0;
    //         let mut input: String = "".to_owned();
    //         for line in lines {
    //             if let Ok(ip) = line {
    //                 // println!("{}", ip);
    //                 if ip.starts_with('>') && i==0 {
    //                     let ip_title = rem_first_and_last(&ip);
    //                     input.push_str(">");
    //                     input.push_str(&ip_title);
    //                     input.push_str("\n");   
    //                     outfile.push_str(&ip_title);
    //                     outfile.push_str(".fasta");
    //                     // writeln!(to_write_file, "{}", input).unwrap();
    //                 }
    //                 else {
    //                     if i==1 {
    //                         input.push_str(&ip);
    //                         input.push_str(":");
    //                     }
    //                     if i==3 {
    //                         input.push_str(&ip);
    //                         //writeln!(to_write_file, "{}", input).unwrap()
    //                     }
    //                 }
    //                 // if ip.starts_with('>') && i==2 {
    //                 //     let ip = rem_first_and_last(&ip);
    //                 //     input.push_str(ip);
    //                 // }
    //             }
    //             i = i + 1;
    //         }
    //         let mut to_write_file = BufWriter::new(
    //             OpenOptions::new()
    //             .append(true)
    //             .create(true) // Optionally create
    //             .open(outfile)
    //             .expect("Unable to open file"),
    //         );
    //         writeln!(to_write_file, "{}", input).unwrap();
    //         // writeln!(to_write_file, "{}", input.to_string() + "\t" + "1.0").unwrap();
    //     }
    // }
}

// impl fmt::Display for Result<String, std::io::Error> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.next_back();
    chars.as_str()
}

fn rem_two_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.next_back();
    chars.as_str()
}