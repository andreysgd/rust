use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let conv = data_input.trim().parse::<i32>().unwrap();
    conv
}

fn main() {
    println!("Type a non-negative integer.");
    let mut integer = String::new();
    io::stdin().read_line(&mut integer).expect("Error reading number.");
    let mut converted_integer = convert_to_int(&integer);

    let mut factorial = 1;

    while converted_integer > 1{
        factorial = factorial * converted_integer;
        converted_integer -= 1;
    }

    println!("The factorial is {}", factorial);
}
