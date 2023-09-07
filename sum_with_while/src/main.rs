use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let conv = data_input.trim().parse::<i32>().unwrap();
    conv
}

fn main() {
    let mut sum = 0;
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Error reading input.");

    let mut entry_converted = convert_to_int(&entry);

    while entry_converted !=0 {
        let rest = entry_converted %10;
        sum = sum + rest;
        entry_converted = entry_converted /10;
    }

    println!("The sum of the numbers is {sum}")
}
