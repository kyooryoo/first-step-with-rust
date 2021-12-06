fn main() {
    println!("Hello, memory!");

    // binds a string to a variable
    let mascot = String::from("ferris");
    // ownershiop is transferred or value is moved
    let ferris = mascot;
    // new variable has the binding or ownership
    // we can borrow its value for print out it
    println!("new var has value: {}", ferris);
    // // old variable lost the binding or ownership
    // // cannot borrow value after it is moved
    // println!("old var has value: {}", mascot);

    let s = String::from("Cannot move twice!");
    // we can clone a complex type value or
    // in a better way, use reference to borrow value
    process_s(s.clone()); // create new value explicitly
    process_s(s); // value moved here
                  // process_s(s); // cannot use value after move

    // simple type value is copied implicitly
    // since Copy trait is implemented for them
    let n = 1u32;
    process_n(n); // value is copied here implicitly
    process_n(n); // value was, and is, not moved

    // create a variable with complex value
    let s = String::from("borrowed!");
    // & borrow is immutable borrow
    // we can read data but cannot change it
    let s_ref = &s; // create a reference
    println!("reference 1: {}", s_ref);
    println!("reference 2: {}", s_ref);

    // use mut keyword if borrowed value will change
    let mut m = String::from("change");
    println!("old value: {}", m);
    // &mut borrow is mutable borrow
    // we can read and change the original data
    change(&mut m);
    println!("new value: {}", m);

    // // CAUTION: &mut
    // // mutable borrow can only happen for once
    // let ref1 = &mut m;
    // let ref2 = &mut m; // compile error here
    // println!("{},{}",ref1,ref2);

    // // even cannot mutable borrow
    // // if immutable borrow already
    // let ref1 = &m;
    // let ref2 = &mut m; // complier complains here
    // println!("{},{}", ref1, ref2);

    // // demonstrate a lifetime issue
    // let x;
    // {
    //     let y = 42;
    //     x = &y; // borrowed value does not live long enough
    // } // 'y' dropped here while still borrowed
    // print!("x: {}", x);   // borrow later used here

    // demonstrate the concept of lifetime
    let x = 32;
    let y = 42;
    let result = greater_value(&x, &y);
    println!("The greater value is: {}", result);

    // demonstrate the lifetime in types
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    drop(text); // drop the original string will cause lifetime issue
    println!("{:?}", fox);
    println!("{:?}", dog);

    // a final exercise for lifetimes
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

fn process_s(input: String) {
    println!("moved: {}", input);
}

fn process_n(input: u32) {
    println!("copied: {}", input);
}

// this function does not return anything
// it just changes the passed in var value
fn change(text: &mut String) {
    text.push_str("!");
}

// following fn definition without lifetime param not work
// fn greater_value(x: &i64, y: &i64) -> &i64 {
fn greater_value<'a>(x: &'a i64, y: &'a i64) -> &'a i64 {
    if x > y {
        x // compiler does not know which one dropped first
    } else {
        y // lifetime param force the same lifetime applied
    }
}
// when a struct or enum holds ref in one of its fields
// type definition with lifetime must get annotated
#[derive(Debug)]
// so the struct lives as long as the passed in string
struct Highlight<'s>(&'s str);

fn copy_and_return<'s>(vector: &mut Vec<String>, value: &'s str) -> &'s str {
    vector.push(String::from(value));
    value
}
