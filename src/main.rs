mod user;


use std::io;


use user::email::Email;
use user::user::User_Struct;

const NATES_EMAIL: &str = "natemiklas1@gmail.com";

fn main() {

    for _ in 0..1 {
        // run a loop precisely x times

        println!("Enter the email for a new user");

        let mut input = String::new(); // creating a new empty string type

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input!");

        if input.trim() == NATES_EMAIL {
            // comparing the strings I guess
            println!("That is Nate's email!")
        }

        let email = Email::Yahoo(input); // creating a new enum with the Yahoo type

        //take a reference to email, check which enum type it is
        match &email {
            Email::Gmail(the_string) => println!("USER IS USING GMAIL! {the_string}"),
            // Email::Yahoo(_email) => println!("USER IS USING YAHOO!"),
            // Email::Aol(_email) => println!("USER IS USING AOL!"),
            other => println!("USER IS NOT USING GMAIL! {other:?}"), // catch-all
        };

        // the below can also be used to do the same as the above

        // if let Email::Gmail(r) = &email{
        //     println!("USER IS USING GMAIL! {r:?}")
        // } else {
        //     println!("USER IS NOT USING GMAIL!")
        // }

        let mut new_user = User_Struct::new(email, String::from("NEWUSER")); // this user needs to be mutable so we can update its email later

        dbg!(&new_user); // debug stmt

        let updated_email = Email::Gmail(String::from("bademail.com")); // setting update_email to be an Email enum of type Gmail

        new_user.update_email(updated_email);

        dbg!(&new_user); // debug stmt

        // let pretty_string = new_user.pretty_print();
    }
}
