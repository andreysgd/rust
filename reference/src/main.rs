fn main() {
    let x = 10;
    let y = &x; // in rust all references are immutable
    let z = x;

    println!("x is {x}, y is {y}, z is {z}");

    let mut a = 20;
    let temp_a = a;
    let b = &mut a;
    *b += 10;

    println!("a was {temp_a}, now it is {b}");
}
