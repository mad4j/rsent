use clap::Parser;
use entropy::Entropy;

use std::fs::File;
use std::io::{self, Read};

mod entropy;
mod index;
mod mean;

use index::Index;
use mean::Mean;

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

    /// treat input as a stream of bits
    #[arg(short, long, default_value_t = false)]
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
    let mut entropy = Entropy::new(args.bit_flag);

    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Impossibile leggere i dati");

    mean.update(&buffer);
    entropy.update(&buffer);

    println!("{:.6}", mean.get_value());
    println!("{:.6}", entropy.get_value());
}
