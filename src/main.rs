use std::{
    io::{stdin, stdout, Write},
    mem,
};

#[allow(dead_code)]
struct Filler {
    // 32 Bytes in total
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Filler {
    pub fn new() -> Self {
        Filler {
            a: 1.0,
            b: 1.0,
            c: 1.0,
            d: 1.0,
        }
    }
}

fn fill_mem(max_el: usize, vec: &mut Vec<Filler>) {
    let mut stdout = std::io::stdout();

    for i in 0..10 {
        print!("\rFilling {}% RAM", (i + 1) * 10);
        stdout.flush().unwrap();

        for _ in 0..(max_el / 10) {
            // Initializes 32 bytes of memory and pushes it into memory
            vec.push(Filler::new());
        }
    }
}

fn main() {
    let max_el = {
        print!("Please enter the amount on memory you would like to fill (megabytes): ");
        stdout().flush().unwrap();
        let mut megabytes = String::new();
        stdin().read_line(&mut megabytes).unwrap();
        megabytes.pop();

        // Converts megabytes input to `usize` type
        let megabytes = megabytes.parse::<usize>().unwrap();

        // Exactly 1 Gigabyte of memory
        let bytes = megabytes * 1024 * 1024;

        bytes / mem::size_of::<Filler>()
    };

    // Create an empty vector, allowing us to push new `Filler`s into memory
    let mut vec = Vec::<Filler>::new();
    fill_mem(max_el, &mut vec);

    println!("\nYour RAM has been filled :)");

    // Wait until the user is ready to clear memory
    {
        let mut stdout = std::io::stdout();

        print!("Press Enter to clear memory");
        stdout.flush().unwrap();
        let mut str = String::new();
        stdin().read_line(&mut str).unwrap();
    }

    // Clear memory instantly
    drop(vec);

    println!("\nMemory Cleared...");
}
