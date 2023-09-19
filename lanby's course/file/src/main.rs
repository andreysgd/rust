use std::fs::File;
use std::io::prelude::*;

fn main() {
    // reading existing file
    let mut open_existing_file = File::open("rust.txt").unwrap();
    let mut content = String::new();
    open_existing_file.read_to_string(&mut content).unwrap();

    println!("{:?}", content);

    // creating file
    let mut create_file = File::create("created.txt").unwrap();
    create_file.write_all(b"Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety-ensuring that all references point to valid memory-without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent data races, its borrow checker tracks the object lifetime of all references in a program during compilation. Rust borrows ideas from functional programming, including immutability, higher-order functions, and algebraic data types. It is popular for systems programming.").unwrap(); // b means byte slice
    
    // reading created file
    let mut open_created_file = File::open("created.txt").unwrap();
    let mut created_content = String::new();
    open_created_file.read_to_string(&mut created_content).expect("Error");
    println!("{:?}", created_content);
}
