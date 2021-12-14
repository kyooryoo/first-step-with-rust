mod auth_mod;
use regex::Regex;

fn main() {
    println!("Hello, codeorg!");

    println!("\nFrom inner authentication module:");
    let mut user = authentication::User::new("jeremy", "super-secret");
    println!("The username is: {}", user.get_username());
    println!("Password Hash is: {}", user.get_password());
    user.set_password("even-more-secret");
    println!("New Password Hash: {}", user.get_password());

    auth_mod::greeting();
    let mut user = auth_mod::User::new("tomy", "not-secret");
    println!("The username is: {}", user.get_username());
    println!("Password Hash is: {}", user.get_password());
    user.set_password("still-not-secret");
    println!("New Password Hash: {}", user.get_password());

    // use a third-party library
    println!("\nFor testing third-party libray:");
    let re = Regex::new(r"^[1-2][0-9]{3}-[a-zA-Z]{3}-[1-3][0-9]$").unwrap();
    println!("2021-Dec-15 match? {}", re.is_match("2021-Dec-15"));

    // exercise of module function and visibility
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}

mod authentication {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(&password),
            }
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn get_password(&self) -> &u64 {
            &self.password_hash
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(&new_password)
        }
    }

    fn hash_password<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

// module could reside in another module
// both inner module and function should be public if necessary
mod text_processing {

    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            // notice how to filter and count chars in a string
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
           text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}