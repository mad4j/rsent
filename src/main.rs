use clap::Parser;

use std::fs::File;
use std::io::{self, Read};

mod index;
mod mean;

use mean::Mean;
use index::Index;

/// evaluate data randomness
#[derive(Parser, Debug)]
#[command(
    author = "Daniele Olmisani <daniele.olmisani@gmail.com>",
    version = "1.0",
    about = "evaluate data randomness"
)]
struct Args {
    /// input data, sdtin if not specified
    input: Option<String>,

    #[arg(short, long, default_value_t=false)]
    bit_flag: bool,
}

fn main() {
    let args = Args::parse();

    let mut reader: Box<dyn Read> = match args.input {
        Some(file) => {
            let file = File::open(file).expect("Impossibile aprire il file");
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    };

    let mut mean = Mean::new(args.bit_flag);

    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Impossibile leggere i dati");

    mean.update(&buffer);

    println!("{:.2}", mean.get_value());

}
