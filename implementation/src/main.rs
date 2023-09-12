struct User {
    username: String,
    email: String,
    active: bool,
    gender: String,
}

impl User {
    fn users_name(&self){
        println!("The user's name is {}", self.username);
    }

    fn users_active(&self){
        println!("Confirm the user is active: {}", self.active);
    }
}

fn main() {
    let user = User{
        username: String::from("Just Ken"), 
        email: String::from("imjustken@gmail.com"), 
        active: true, 
        gender: String::from("Male")
    };

    user.users_name();
    user.users_active();

/*     println!("user.users_name");
    println!("user.users_active"); */
}
