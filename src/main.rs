extern crate regex;
use dialoguer::Input;
use regex::Regex;


/// Checks whether the token consists of a required set of characters
fn token_first_validation(token: &String) -> bool {
    let re = Regex::new("^[a-z0-9]{85}$").unwrap();
    re.is_match(token)
}

fn main() {
    println!("Hello world!");
}