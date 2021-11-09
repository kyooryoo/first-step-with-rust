#[derive(Debug)]
// annotate a struct with a lifetime called 'document.
// Highlight struct can't outlive the source of the &str that it borrows
struct Highlight<'document>(&'document str);
fn erase(_: String) { }

// for the exercise
fn copy_and_return<'a>(vector:&'a mut Vec<String>, value:&'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len()-1).unwrap()
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    // create two instances of the Highlight struct. 
    // each reference to a segment of the String value owned by `text`
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    // // cannot move out of `text` but borrow it later with fox and dog
    // erase(text);

    // highlight goes out of scope before text goes out of scope.
    // this means that the Highlight instance is valid.
    println!("{:?}", fox);
    println!("{:?}", dog);
    
    
    // exercise
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";
    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));
    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
    
}