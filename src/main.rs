
struct User {
    username: String,
    email: String,
    is_alive: bool,
    sex: String
}


impl User {
    fn user_name(&self) {
        println!("Username is: {}", self.username);
    }

    fn is_alive(&self) {
        println!("Is the user alive? {}", self.is_alive);
    }
    fn email(&self) {
        if self.is_alive == false {
            println!("The late user's email is {}", self.email);
        } else {
            println!("User's email is {}", self.email);
        }
       }
    fn sex(&self) {
        println!("User's sex is {}", self.sex);
    }
}




fn main() {
    let pessoa = User{username: String::from("joao23"), email: String::from("joaosucks@joaoblowjob.com"), is_alive: bool::from(false), sex: String::from("Male")};
    pessoa.user_name();
    pessoa.is_alive();
    pessoa.email();
    pessoa.sex();
}
