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
    }}

fn hash_password<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn greeting() {
    println!("\nFrom outer authetication module:");
}