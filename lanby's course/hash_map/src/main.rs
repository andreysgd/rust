use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();
    hash_map.insert("CS", 50);
    hash_map.insert("CE", 62);
    hash_map.insert("SA", 105);
    hash_map.insert("SE", 186);

    println!("{:?}", hash_map);

    match hash_map.get("SE"){
        Some(k) => println!("SE stands for Software Engeneering, as its code is {}", k),
        None => println!("There's no such thing as SE")
    }

    hash_map.remove("SA");
    println!("{:?}", hash_map);
    println!("Is there a SA? {}", hash_map.contains_key("SA"));
}
