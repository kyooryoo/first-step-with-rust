fn main() {
    println!("Hello, array!");

    // array can be defined by an item list
    // use signature [T;size] optionally
    let days:[&str;7] = ["Sun","Mon","Teu","Wed","Thu","Fri","Sat"];
    // or be defined by initial value and length
    let bytes = [0;5]; // creates five 0s in this array
    // days[7] or bytes[5] cause index out of bounds error
    println!("The first day of week is {}", days[0]);
    // use {:?} to display all array values
    println!("All array values are: {:?}", bytes);

    // declare vector with vec! macro
    // use the signature Vec<T> optionally
    let mut three_num:Vec<u32> = vec![15,4,56];
    println!("Vector values: {:?}", three_num);
    three_num[1] += 20;
    println!("New 2nd vlaue: {}", three_num[1]);
    // declare vector with Vec::new() method
    // the default data type is &str
    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Orange");
    println!("My favorite fruits: {:?}", fruit);

    // index out of bounds happens in runtime
    // println!("NOT at compile time: {}", fruit[10])

    // different data type cannot get pushed
    // fruit.push(1); // compile error

    println!("Pop off: {:?}", fruit.pop());
    println!("New fruits: {:?}", fruit);
    println!("Top fruit: {}", fruit[0]);
}
