use std::io;

#[derive(Debug)]
struct User {
    email: Email, //using the Email enum here
    username: String,
    age: u32,
}

// impl block to give functions / methods to the User struct
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

        let email = Email::Yahoo(input); // creating a new enum

        //take a reference to email, check which enum type it is
        match &email {
            Email::Gmail(email) => println!("USER IS USING GMAIL! {email:?}"),
            // Email::Yahoo(_email) => println!("USER IS USING YAHOO!"),
            // Email::Aol(_email) => println!("USER IS USING AOL!"),
            other => println!("USER IS NOT USING GMAIL! {other:?}") // catch-all
        };

        // the below can also be used to do the same as the above

        // if let Email::Gmail(r) = &email{
        //     println!("USER IS USING GMAIL! {r:?}")
        // } else {
        //     println!("USER IS NOT USING GMAIL!")
        // }

        let mut new_user = User::new(email, String::from("NEWUSER")); // this user needs to be mutable so we can update its email later

        dbg!(&new_user); // debug stmt

        let updated_email = Email::Gmail(String::from("bademail.com")); // setting update_email to be an Email enum of type Gmail

        new_user.update_email(updated_email);

        dbg!(&new_user); // debug stmt

        // let pretty_string = new_user.pretty_print();
    }
}
