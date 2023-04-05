use std::{
    io::{stdin, Write},
    mem,
};

use clap::Parser;
use rayon::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    #[clap(help = "The number of megabytes to allocate")]
    megabytes: usize,
}

// 64 Bytes in total
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Filler(u128, u128, u128, u128);

fn main() {
    let args = Args::parse();
    let max_el = {
        // Convert megabytes to bytes
        let bytes = args.megabytes * 1024 * 1024;

        bytes / mem::size_of::<Filler>()
    };

    // Create an empty vector, allowing us to push new `Filler`s into memory
    let vec = vec![(); max_el];
    let filled: Vec<Filler> = vec.into_par_iter().map(|_| Filler::default()).collect();

    println!("\nYour RAM has been filled :)");

    // Wait until the user is ready to clear memory
    let mut stdout = std::io::stdout();

    print!("Press Enter to clear memory");
    stdout.flush().unwrap();
    let mut str = String::new();
    stdin().read_line(&mut str).unwrap();

    // Clear memory "instantly"
    drop(filled);

    println!("\nMemory Cleared...");
}
