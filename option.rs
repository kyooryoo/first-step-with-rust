fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            // match arms are evaluated from top to bottom.
            // Specific cases must be defined earlier than generic cases
            // or they'll never be matched and evaluated.
            // match arms must cover every possible input value.
            // compiler error happens when match against a non-exhaustive.
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let a_number: Option<u8> = Some(7);
    match a_number {
        // _ wildcard match anything else, so the compiler exhaust match arms.
        Some(7) => println!("\nMy lucky 7 from match!"),
        _ => {}
    }

    // assign Some(7) without specifying Option type as follows
    let a_number = Some(7);
    if let Some(7) = a_number {
        println!("My lucky 7 from if let!");
    }

    // unwrap method retrieves the inner value of an option type
    // the type `Option<&str>` for `Some("candy")` is NOT mandatory
    let gift: Option<&str> = Some("candy");
    assert_eq!(gift.unwrap(), "candy");
    // the explicit type `Option<&str>` for `None` is mandatory
    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    // expect does the same as unwrap but provides custom panic message
    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");
    let b: Option<&str> = None;
    b.expect("fruits are healthy"); // panic for None as well
    
    // unwrap_or returns Some value or the specified value for None
    // match None case explicitly or use unwrap_or to avoide panic
    assert_eq!(Some("dog").unwrap_or("cat"), "dog"); // returns dog
    assert_eq!(None.unwrap_or("cat"), "cat"); // returns cat
}
