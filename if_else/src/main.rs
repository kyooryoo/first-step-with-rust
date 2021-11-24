use std::io;

fn main() {
    println!("Hello, conditions!");

    let mut input = String::new();
    println!("What weekday is today?");

    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read user input!");

    let weekday: &str = input.trim();
    let weekday = weekday.to_lowercase();
    println!("\nFrom if else branch:");
    if weekday.eq("friday") || weekday.eq("fri") {
        println!("Have a good weekend!")
    } else {
        println!("Another working day!")
    }

    // if else works as an expression returns value
    // branches must return the same data type values
    let greeting = if weekday.eq("friday") || weekday.eq("fri") {
        "Have a good weekend!"
    } else {
        "Another working day!"
    };
    println!("\nFrom if else expression:\n{}", greeting);

    // combine multiple conditions into one statement
    println!("\nFrom mutiple branches:");
    if weekday.eq("mon") || weekday.eq("monday") {
        println!("Have a good week!");
    } else if weekday.eq("fri") || weekday.eq("friday") {
        println!("Have a good weekend!");
    } else {
        println!("Have a good weekday!");
    }

}
