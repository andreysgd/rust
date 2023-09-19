use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let conv = data_input.trim().parse::<i32>().unwrap();
    conv
}

fn main() {
    let mut number_x = String::new();
    let mut number_y = String::new();

    println!("Type the first number.");
    io::stdin().read_line(&mut number_x).expect("Error reading number.");

    println!("Type the second number.");
    io::stdin().read_line(&mut number_y).expect("Error reading number.");

    if convert_to_int(&number_x) > convert_to_int(&number_y){
        println!("The number {} is bigger than {}", number_x, number_y);
    } else {
        println!("The number {} is less or equal {}", number_x, number_y);
    }

}
