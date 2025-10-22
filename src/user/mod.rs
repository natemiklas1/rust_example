pub mod user {

    use super::email::Email;

    #[derive(Debug)]
    pub struct User_Struct {
        pub email: Email, //using the Email enum here
        pub username: String,
        age: u32,
    }

    // impl block to give functions / methods to the User struct
    impl User_Struct {
        // An assoicated function that is NOT a method
        pub fn new(email: Email, username: String) -> Self {
            Self {
                email, // these two fields are created using the field init shorthand, since the parameter name is the same as the field name
                username,
                age: 0, // making this default to 0 for no reason
            }
        }

        pub fn update_email(&mut self, new_email: Email) {
            // need a &mut to be able to update at the reference of self
            self.email = new_email
        }
    }
}

pub mod email;
