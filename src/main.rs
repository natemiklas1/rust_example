mod Account;
use std::io;

use Account::email::email::Email;
use Account::user::User_Struct;

const NATES_EMAIL: &str = "natemiklas1@gmail.com";

fn main() {
    let mut user_vector: Vec<User_Struct> = Vec::new();

    // how to write a vector that would have values at initialization
    // let v = vec![user1, user2, user3, etc...]

    for _ in 0..2 {
        // run a loop precisely x times

        println!("Enter the email for a new user");

        let mut input = String::new(); // creating a new empty string type

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input!");

        let trimmed_input = String::from(input.trim());

        if trimmed_input == NATES_EMAIL {
            // comparing the strings I guess
            println!("That is Nate's email!")
        }

        let email = Email::Yahoo(trimmed_input); // creating a new enum with the Yahoo type

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

        let new_user = User_Struct::new(email, String::from("NEWUSER")); // this user needs to be mutable so we can update its email later

        dbg!(&new_user); // debug stmt

        //new_user.update_email(updated_email);

        // push user struct onto the new vector
        user_vector.push(new_user);

        // iterate over the vector with a mutable referance to each item so we can update!
        for i in &mut user_vector {
            i.add_other_email(Email::Gmail(String::from("thisemailsucks.com")));
            println!("{i:?}")
        }
    }
}
