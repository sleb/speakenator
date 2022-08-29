use std::{fs::File, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    history_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let f = File::open(&args.history_file).unwrap();
    let parser = dsv::Parser::new();

    for row in parser.parse(f) {
        println!("{:?}", row);
    }
}
