use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let conv = data_input.trim().parse::<i32>().unwrap();
    conv
}

fn main() {
    println!("Type the first number.");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Error reading first number.");
    let mut converted_x = convert_to_int(&mut x);
    let x_trimmed = x.trim();

    println!("Type the second number.");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Error reading second number.");
    let mut converted_y = convert_to_int(&mut y);
    let y_trimmed = y.trim();

    while converted_y != 0{
        let temp = converted_y;
        converted_y = converted_x % converted_y;
        converted_x = temp;
    }

    println!("The greatest common divisor of {x_trimmed} and {y_trimmed} is {converted_x}");
}
