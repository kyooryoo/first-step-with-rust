fn main() {
    println!("Hello, error!");

    // // call panic macro 
    // panic!("Farewell!");

    let v = vec![0,1,2,3,4];
    // // index out of bounds causes panic
    // println!("{}", v[5]);

    // // the enum model for optional string
    // enum Option<T> {
    //     None,       // if value doesn't exist
    //     Some(T),    // if value exists
    // }

    // Vec::get implements the above model
    let exist = v.get(4);
    println!("\nIf exist, you get: {:?}", exist);
    // use unwrap get value from the some variant
    println!("exist.unwrap(): {}", exist.unwrap());
    
    // Vec::get return None for not existing value
    let not_exist = v.get(5);
    println!("\nIf not exist, you get: {:?}", not_exist);
    println!("You do not need to unwrap the None!");

    // use match to find a matching pattern
    // notice match is performed by the order of arms
    println!("\nFor the vector: {:?}", v);
    for index in [0, 3, 5] { // for &index in [0, 3, 5].iter() {
        match v.get(index) {
            // match Some(3) firstly
            Some(3) => println!("it's 3 in index {}!", index),
            // then match Some(num) secondly
            Some(num) => println!("value in index {}: {}", index, num),
            // then match None thirdly
            None => println!("No value in index {}!", index),
        }
    }

    // define an Option variant some() and match it specifically
    println!("\nMatch to some(value) directly!");
    let a_number: Option<u8> = Some(3);
    match a_number {
        Some(5) => println!("Match to 5!"),
        Some(3) => println!("Match to 3!"),
        // notice the keyword _ is the wildcard pattern
        // it matches all possible other conditions 
        _ => {},
    }

    // use if let operator for SINGLE pattern match
    // it mathces pattern to expression that returns value
    // notice put pattern on left and expression on right
    if let Some(3) = a_number {
        println!("Match to 3, again!");
    } else {
        println!("Not match to 3!");
    }

    // access inner value in Option type with unwrap()
    assert_eq!(&3, v.get(3).unwrap());
    assert_eq!(Option::None, v.get(5));
    
    // // notice unwarp a None value will cause panic
    // assert_eq!(Option::None, v.get(5)); // wll panic

    // use unwrap_or() to handle possible None value
    // following code defaults None to the number 0
    assert_eq!(&0, v.get(5).unwrap_or(&0));
    // following code test unwrap_or returned values
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");

    // use expect to customize panic message
    assert_eq!(v.get(3).expect("No panic!"), &3);
    // // expect cannot unwrap the None value
    // // so it causes panic with custom message
    // v.get(5).expect("Panic here!");

    // following exercise play with absent value
    let john = Person {
        first_name: String::from("James"),
        middle_name: Some(String::from("Oliver")),
        last_name: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first_name: String::from("Alice"),
        middle_name: None,
        last_name: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");
}

// for exercise
struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

// for exercise
fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first_name);
    full_name.push_str(" ");
    match &person.middle_name {
        Some(middle_name) => {
            full_name.push_str(middle_name);
            full_name.push_str(" ");
        },
        _ => {}
    }
    full_name.push_str(&person.last_name);
    full_name
}
