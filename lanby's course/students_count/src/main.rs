use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let conv = data_input.trim().parse::<i32>().unwrap();
    conv
}

fn main() {
    println!("Type the average quantity.");
    let mut average_quantity = String::new();
    io::stdin().read_line(&mut average_quantity).expect("Error reading average quantity.");
    let mut students_below_average = 0;
    let mut iter = 1;

    while convert_to_int(&average_quantity) >= iter {
        println!("Type the student average number {iter}.");
        let mut student_average = String::new();
        io::stdin().read_line(&mut student_average).expect("Error reading student's average.");
        iter += 1;

        if convert_to_int(&student_average) <= 6 {
            students_below_average += 1;
        }
    }

    println!("The number of students bellow average is {students_below_average}")
}
