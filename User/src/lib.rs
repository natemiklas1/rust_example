pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


mod User {

    #[derive(Debug)]
    pub struct User {
        pub email: Email, //using the Email enum here
        pub username: String,
        age: u32,
    }

    // impl block to give functions / methods to the User struct
    impl User {
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

    // fn pretty_print(&self) -> String {
    //     String::from(format!(
    //         "PRETTY PRINT!\nEmail: {}, Username: {}, Age: {}",
    //         self.email, self.username, self.age
    //     ))
    // }
    }

    // email enum
    #[derive(Debug)]
    pub enum Email {
        Gmail(String),
        Yahoo(String),
        Aol(String),
    }

}
