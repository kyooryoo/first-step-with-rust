fn main() {
    println!("Hello, function!");

    hello();
    println!("100 divide by 3 is: {}", divide_by(100.0, 3.0));
    goodbye("Friend");
}

// a function with no input args and no return
fn hello() {
    println!("Hello There!");
}

// a function with input arg but no return
// notice input arg has data type specified
fn goodbye(name:&str) {
    println!("Goodbye {}!", name);
}

// a function with both input arg and return
// Use *-> <type>* to specify returned value.
// Notice input and output should be both typed.
fn divide_by(x:f64, y:f64) -> f64 {
    if y == 0.0 {
        // Return value wit *return* has *;* as ending.
        return 0.0;
    }
    // Return value directly has no *;* as ending.
    x / y
}