struct User (String, String, bool, String);

fn main() {
    let mut user = User (String::from("Josh Pett"), String::from("joshpett@gmail.com"), true, String::from("joshpett"));
    println!("The user's name is {}. His e-mail is {}, with an active status of {}. Find him by the following username: {}.", user.0, user.1, user.2, user.3);
    user.0 = String::from("Joshua Pett");
    println!("Actually, his name is {}!", user.0)
}
