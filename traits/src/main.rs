fn main() {
    println!("Hello, traits!");

    println!("\nStruct<T> - One for all generic type:");
    let boolean = Point {
        x: true,
        y: false,
        z: 30,
    };
    let integer = Point {
        x: 10,
        y: 20,
        z: 30,
    };
    let float = Point {
        x: 10.0,
        y: 20.0,
        z: 30,
    };
    let string_slice = Point {
        x: "high",
        y: "low",
        z: 30,
    };
    // for printing out trait detail, add [derive(Debug)]
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}",
        boolean, integer, float, string_slice
    );
    // for comparing struct, add [derive(PatialEq)]
    if integer == integer {
        println!("Equal!")
    }

    // // mixing types in the first struct does not work
    // let wont_work = Point { x: true, y: 20.0, z: 30  };\

    // with specifically defined types for fields, it works now:
    println!("\nStruct<T, U> - Mixed generic types:");
    let int_and_bool = Points {
        x: 10,
        y: false,
        z: 30,
    };
    let bool_and_float = Points {
        x: true,
        y: 20.0,
        z: 30,
    };
    println!("{:?}\n{:?}", int_and_bool, bool_and_float);

    // demonstrate implemented traits
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    println!("Circle area: {}", circle.area());
    println!("Rectanble area: {}", rectangle.area());

    // test area() from trait bounds
    println!("Circle area: {}", area(&circle));
    println!("Rectanble area: {}", area(&rectangle));

    // test the self implemented next function
    let mut counter = Counter::new(6);
    println!("Counter created:\n{:#?}", counter);
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
    println!("Counter exhausted:\n{:#?}", counter);

    // use for to loop through the counter
    for num in Counter::new(10) {
        println!("{}", num);
    }

    // other methods implemented by Iterator
    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);
    let powers_of_2: Vec<usize> = Counter::new(10).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);

    // for testing exercise 1 result
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));

    // for testing exercise 2 result:
    let data = vec!['4', '1', '1', '2', '1', '3', '3', 'A', 'A', 'A', '5', '5'];
    assert_eq!(
        Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec!['4'],
            vec!['1', '1'],
            vec!['2'],
            vec!['1'],
            vec!['3', '3'],
            vec!['A', 'A', 'A'],
            vec!['5', '5'],
        ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    assert_eq!(
        Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
        vec![
            vec![1],
            vec![2, 2],
            vec![1, 1],
            vec![2, 2],
            vec![3],
            vec![4, 4],
            vec![3],
        ]
    )
}

// one type for all fields
#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
    z: usize,
}

// specific types for fields
#[derive(Debug, PartialEq)]
struct Points<T, U> {
    x: T,
    y: U,
    z: usize,
}

// a trait for calculating Area
trait Area {
    fn area(&self) -> f64;
}
// two structs that implement trait
struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}
// implement the trait
impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}
impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// define an area() for demo trait bounds
fn area(value: &impl Area) -> f64 {
    value.area()
}

// // Rust defines the Iterator trait by default
// trait Iterator {
//     // the type Item will be the return type
//     // as a place holder it can be any type
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// create a struct to hold the iterator state
#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}
// create a new method for initializing the struct
impl Counter {
    fn new(length: usize) -> Counter {
        Counter { length, count: 0 }
    }
}

// add fn next explicitly on Iterator for Counter
impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

// exercise 1: using generic type:
// create a Container can hold and retur any type
struct Container<T> {
    value: T,
}
// notice *<T>* is also added to *impl*
impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

// exercise 2: for trait and iterator
#[derive(Debug)]
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
        Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }
        let items = self.inner.drain(0..cursor).collect();
        Some(items)
    }
}
