struct User {
    username: String,
    email: String,
    active: bool,
    gender: String
}

fn user(user: User){
    println!("The name of the user is {}", user.username)
}

fn main() {

    let mut user_01 = User {
        username:String::from("Jean Starless"),
        email:String::from("jeanstarless@gmail.com"),
        active:true,
        gender:String::from("Male")
    };
    user_01.active = false;
    println!("User's name is {}", user_01.username);
    println!("User's email is {}", user_01.email);
    println!("User's gender is {}", user_01.gender);
    println!("Checking if user {} is active: {}", user_01.username, user_01.active);

    user(user_01);
}
