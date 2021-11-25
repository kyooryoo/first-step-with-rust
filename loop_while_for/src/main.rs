fn main() {
    println!("Hello, interations");

    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };
    println!("Loop breaks at: {}", stop_loop);

    counter = 1;
    while counter < 100 {
        counter *= 2;
        println!("Current counter is: {}", counter);
    }

    let fruits = ["Apple", "Banana", "Orange"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits.iter(): {:?}", fruits.iter());
    for fruit in fruits {
        println!("We have {}!", fruit);
    }
    for i in 0..5 {
        println!("Loop through num: {}", i);
    }
    for j in 'm'..'z' {
        println!("Loop through char: {}", j);
    }
    for c in "strings".chars() {
        println!("Loop through str: {}", c);
    }
}
