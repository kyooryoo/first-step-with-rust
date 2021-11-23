fn main() {
    println!("Hello, vector!");

    // declare vector with vec! macro
    // element values assigned like array
    let seven = vec![0,1,2,3,4,5,6];
    // we will change element value later
    let mut zeros = vec![0;5];
    
    // use syntax {:?} to display vectors 
    println!("Initial seven: {:?}", seven);
    println!("Initial zeros: {:?}", zeros);

    // create an empty vector with Vec::new()
    let mut fruit = Vec::new();
    // push value and data type into vector
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("My favorite fruits: {:?}", fruit);
    // pop off and return an element
    println!("Maybe not: {:?}", fruit.pop());
    println!("New favorites: {:?}", fruit);

    // access vector element with index
    println!("The last num of seven: {}", seven[6]);
    zeros[0] = zeros[0] + 1;
    println!("Not zeros: {:?}", zeros);
}
