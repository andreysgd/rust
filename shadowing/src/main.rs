fn main() {
    let a: u8 = 10;
    println!("{a}");

    {
        println!("{a}");
        let a: f32 = 20.5;
        println!("{a}");
    }
    println!("{a}");
}
