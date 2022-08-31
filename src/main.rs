use clap::Parser;
use speak::{list_speakers, Args};

fn main() {
    let args = Args::parse();

    if let Err(e) = list_speakers(&args) {
        eprintln!("Error: {}", e);
    }
}
