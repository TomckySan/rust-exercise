use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f_name = match env::args().nth(1) {
        Some(f_name) => f_name,
        None => {
            println!("Error: No args.");
            return;
        }
    };

    let f = match File::open(f_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Error: Cannot open the file. {}", e);
            return;
        }
    };

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let l = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Error: Cannot read a line. {}", e);
                return;
            }
        };
        println!("{}", l);
    }
}
