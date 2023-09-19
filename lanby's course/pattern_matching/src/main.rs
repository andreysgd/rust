use std::io;

fn main() {
    
    let number = 6;

    match number{
        1 => println!("Number one."),
        2 => println!("Number two."),
        3 | 4 => println!("Number three or four."),
        5..=10 => println!("Number between five and ten."),
        _ => println!("Number not in list.")
    }

    let name = "Carl";

    match name{
        "Ethan" => println!("Lawyer"),
        "Pete" => println!("Comediant"),
        "Lion" => println!("Nurse"),
        _ => println!("No such name in list.")
    }

    println!("Type your occupation.");
    let mut input = String::new();

    // match by reading input from user
    match io::stdin().read_line(&mut input){
        Ok(_) => println!("Sucess. {} enlisted.", input.trim()),
        Err(e) => println!("Failed to enlsit. Error: {e}.")
    }

}
