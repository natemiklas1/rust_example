use std::io;

#[derive(Debug)]
struct User {
    email: Email, //using the Email enum here
    username: String,
    age: u32,
}

// impl black to give functions / methods to the User struct
impl User {
    // An assoicated function that is NOT a method
    fn new(email: Email, username: String) -> Self {
        Self {
            email, // these two fields are created using the field init shorthand, since the parameter name is the same as the field name
            username,
            age: 0, // making this default to 0 for no reason
        }
    }

    fn update_email(&mut self, new_email: Email) {
        // need a &mut to be able to update at the reference of self
        self.email = new_email
    }

    // fn pretty_print(&self) -> String {
    //     String::from(format!(
    //         "PRETTY PRINT!\nEmail: {}, Username: {}, Age: {}",
    //         self.email, self.username, self.age
    //     ))
    // }
}

// email enum
#[derive(Debug)]
enum Email {
    Gmail(String),
    Yahoo(String),
    Aol(String),
}

fn main() {
    for _ in 0..1 {
        // run a loop precisely x times

        println!("Enter the email for a new user");

        let mut input = String::new(); // creating a new empty string type

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input!");

        if input.trim() == "natemiklas1@gmail.com" {
            // comparing the strings I guess
            println!("That is Nate's email!")
        }

        let email = Email::Gmail(input); // creating a new enum

        let mut new_user = User::new(email, String::from("NEWUSER")); // this user needs to be mutable so we can update its email later

        dbg!(&new_user); // debug stmt

        let updated_email = Email::Gmail(String::from("bademail.com"));
        
        new_user.update_email(updated_email);

        dbg!(&new_user); // debug stmt

        // let pretty_string = new_user.pretty_print();
    }
}
