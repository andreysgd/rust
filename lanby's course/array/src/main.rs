fn main() {

    println!("{}", "*".repeat(50));
    println!("Creating an array.");
    println!("{}", "*".repeat(50));

    let array_01 = [1, 2, 3, 4, 5];
    println!("First array:");
    println!("{:?}", array_01);
    println!("Second value in the array:");  
    println!("{}", array_01[2]);
    println!("Iterating the first array:");
    for i in 0..array_01.len(){
        println!("{}", array_01[i]);
    }

    println!("{}", "*".repeat(50));
    println!("Using 'iter' function to iterate an array.");
    println!("{}", "*".repeat(50));

    println!("Second array:");
    // explicits type and lenght of array
    let array_02 : [u32; 5] = [6, 7, 8, 9, 10];
    println!("Iterating the second array, this time with iter():");
    for i in array_02.iter(){
        println!("{}", i);
    }

    println!("{}", "*".repeat(50));
    println!("Reversing an array.");
    println!("{}", "*".repeat(50));

    println!("Third array:");
    // to reverse an array you can create a vector
    let array_03 : [i32; 5] = [11, 12, 13, 14, 15]; 
    println!("{:?}", array_03);
    println!("Reverseing third array:");
    let array_reversed_vector: Vec<i32> = array_03.iter().copied().rev().collect();
    for i in array_reversed_vector.iter(){
        println!("{}", i);
    }

    println!("{}", "*".repeat(50));
    println!("Displaying a value in an array multiple times:");
    println!("{}", "*".repeat(50));
 
    let array_04 = [2; 5]; // integer 2 thirty times]
    
    for i in array_04.iter(){
        println!("{}", i);
    }
}
