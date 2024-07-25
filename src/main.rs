mod convert;
use convert::convert;

use std::{env, fs::read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Only one argument is supported!");
        return;
    }

    let input_path = args.last().unwrap();
    if !input_path.ends_with(".pdf") {
        println!("Only PDF files are supported!");
        return;
    }

    convert(read(input_path).expect("error reading input file"))
        .save(input_path.replace(".pdf", ".png"))
        .expect("error saving output file");
}
