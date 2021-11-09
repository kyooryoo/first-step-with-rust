fn main() {
    compare(1,1);
    compare(1,2);
    
    println!("{}",greeting(false));
}

fn compare(x:u32,y:u32) {
    if x == y {
        println!("equal");
    } else {
        println!("not equal");
    }
}

fn greeting(formal:bool) -> &'static str {
    if formal {
        "Nice to meet you!"
    } else {
        "Good day!"
    }
}