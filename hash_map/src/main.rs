use std::collections::HashMap;

fn main() {
    println!("Hello, Hash Map!");

    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("The history of Japan"), String::from("Koizumi"));
    reviews.insert(String::from("The history of USA"), String::from("Franklin"));
    reviews.insert(String::from("The history of UK"), String::from("Shakespeare"));

    let book: &str = "The history of Japan";
    // notice the escape for new line \n and single quotation mark \'
    // use {:?} for displaying and get method for retrieving hash map values
    // and get method returns a some() object requires unwrap the true value
    println!("\nReview for \'{}\' by {:?}", book, reviews.get(book).unwrap());

    println!("\nThe whole reviews Hash Map:\n{:?}", reviews);
    reviews.remove("The history of USA");
    println!("\nAfter removing one element:\n{:?}", reviews);
}
