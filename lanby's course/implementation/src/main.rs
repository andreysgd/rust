struct User {
    username: String,
    email: String,
    active: bool,
    gender: String,
}

impl User {
    fn users_name(&self){
        println!("User's name is {}", self.username);
    }

    fn users_active(&self){
        println!("Confirm the user is active: {}", self.active);
    }

    fn users_email(&self){
        println!("User's e-mail is {}", self.email);
    }

    fn users_gender(&self){
        println!("User's gender is {}", self.gender);
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
    user.users_email();
    user.users_gender();
    user.users_active();

}
