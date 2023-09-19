
fn main() {
 let mut vectors = vec![1, 2, 3, 4];
 println!("{:?}", vectors);
 vectors.push(5);
 println!("{:?}", vectors[4]);
 println!("{:?}", vectors);
 vectors.remove(1);
 println!("{:?}", vectors);

 for i in vectors.iter(){
    println!("{:?}", i);
 }
}
