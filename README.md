# First Step with Rust
From the [Rust Learning Path](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/) on Microsoft Learn

## Introduction
* Rust is a safe alternative to  C and C++. Like C and C++
* Rust doesn't have a large runtime or garbage collector
* Rust prevents bugs from incorrect memory use > C and C++

### Advantages
* Type Safe: Compiler ensures all variables have correct type
* Memory Sare: References always refer to valid memory
* Date Race Free: Borrow prevents multi*mutate an variable
* Zero-cost Absctraction: High-level features have low cost
* Minimal Runtime: Small runtime and no garbage collector
* Targets Bare Metal: Suitable for OS and Device Drivers

### Rust Module System
Rust manage and orgalize code with the following items:
* Crates: The smallest piece of code the compiler can run
* Modules: Manage the scope of items in side a crate
* Path: For naming items in code and contral privacy

### Std and 3rd-party Libs
Standard library and other 3rd-party libs from [crates.io](https://crates.io/):
* std: The standard library with following modules:
    + std::collections - For collection types, such as HashMap.
    + std::env - Work with system environment.
    + std::fmt - Control output format.
    + std::fs - Work with the file system.
    + std::io - Work with input/output.
    + std::path - Work with file system path data.
* structopt - 3rd-party crate for parsing command-line arguments.
* chrono - 3rd-party crate to handle date and time data.
* regex - 3rd-party crate to work with regular expressions.
* serde - 3rd-party crate to se-deserialize data structures.
Import modules with `use std::fmt` or `use regex::Regex`.

### Manage Project
Create and manage project, and dependencies with Cargo:
* cargo new: Create new project
* cargo build: Build a project
* cargo run: Build and Run a project
* cargo test: Test a project
* cargo check: Check project types
* cargo doc: Build documentation for a project
* cargo publish: Publish a library to crates.io
* cargo.toml: Manage dependent crates

### Rust Playground
Use [it](https://play.rust-lang.org/) to try some code, check syntax, or share code:
There are some build in tools and dev features:
* Format Code: Rustfmt tool can auto format code style
* Test Code: Clippy tool run *lint* to find errors
* Save Code: Auto saved in browser local storage
* Share Code: Create GitHub gist for sharing
Provide the following build options:
* Run: Build and Run the code = cargo run
* Build: Build but don't run = cargo build
* Test: Build and run Test = cargo test
There are also some limitations:
* Network: when compile or run, network connection down
* Memory: available memory is limited for compile and run
* Execute Time: compile and run may timeout if too long
* Disk: disk space for compile and run is also limited
Time to Play:
1. Go to [Rust Playground](https://play.rust-lang.org/)
2. Enter the following code:
`fn main(){println!(Welcome to Rust!);}`
3. Format the code: Tools > Rustfmt
4. Check mistakes: Tools > Clippy
5. Fix it with adding quote marks:
```
fn main() {
    println!("Welcome to Rust!");
}
```
6. Build and Run by choosing *RUN*
7. Click *SHARE* and use **Permalink to the playground**

---

## Environment
Edit *.rs* source file with some IDE such as VS Code
Compile code with `rustc` or Cargo

### Install in Windows
1. Install [Visual Studio Vode](https://code.visualstudio.com/download)
2. Install [Visual C++ buld tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
3. Install `rustup` from [rustup.rs](https://rustup.rs/)
* Update `rustup` with command `rustup update`
4. Check installation with `rustc --version` and `cargo --version`

### Hello World
Create the first app in terminal:
```
$ mkdir hello-world
$ cd hello-world
$ touch main.rs
$ echo "fn main() { println!("Hello, World!"); }" > main.rs
$ rustc main.rs
$ .\main.exe
```

Or use the command in PowerShell:
```
$ New-Item -Path . -Name "main.rs" -ItemType "file" `
>> -Value "fn main() { println!(`"Hello, World!`"); }"
$ rustc main.rs
$ .\main.exe
```

Or use Cargo:
```
$ cargo new hello-cargo
$ cd .\hello-cargo\
$ cargo run
```

## Basic Program Structure
* Function is defined with *fn*
* Program starts from *main* function
* *()* after function name holds args
* *{}* after *()* holds function body
* Most code statements end with a *;*
* Organize code blocks with indentation
* Use *todo!* macro for unfinished code
* Use *println!* macro for screen output
* *{}* for value substitution in string
* Use prefix *//* for inline comment
* Use *""* to quote string typed value

Take the following code as an example:
```
fn main() {
    println!("Hello, {}!", "World");
    greeting("Rust");
}

fn greeting(name) {
    // will be implemented later
    todo!("Greeting {}", name);
}
```
It will generate the Error:
> thread 'main' panicked at 'not yet implemented:
> Greeting Rust', src/main.rs:8:5
And the following Output:
> Hello, World!

### As a typed language
Compiler must know data type to run
* Variable is declared with keyword *let*
* Use *mut* keyword for mutable variable
* Var in the same name shadows prevoius
* Use *:* after a var for its data type

#### Integers
Integers are defined by size and sign
Size defines the bit size of the integer
Sign defines if theint could be negative
* size: 8 / 16 / 32 / 64 / 128 / size
* sign: i / u
For example *isize* means:
* A signed integer could be negative
* Use CPU architecture (32/64) for size
System use *i32* as the default type

#### Floating-point
Theare are two kinds of floating-point data:
* f32 and f64, while f64 is the default.
Use *<value><type>* for println with specified type
Or println will use i32 or f64 as the default type

```
fn main() {
    // var is immutable by default
    let mut num: u32 = 14;
    println!("The number is: {}", num);
    num = 24;
    println!("New number is: {}", num);
    // avoid the default data type
    println!("An integer: {}", 9u32);
    println!("A floating-point: {}", 9.0f32);
}
```

#### Booleans
Boolean means *true* or *false*
* The boolean type has values *true* and *false*
* It is often returned by a comparison check
See example code as follows:
```
fn main() {
    println!("The value of true: {}", true);
    println!("4 > 2 ?: {}", 4 > 2);
    println!("The value of false: {}", false);
    println!("4 < 2 ?: {}", 4 < 2);
}
```

#### Text
There are two types for text: Char and String
* The *char* type data is a single item
* A string contains a series of characters.
* Single quotation is used for *char* values.
* Saved with 21 bit integer (padded to 32 bit).

* The *str* type is actually a view of string.
* Usually, *&str* is used for reference means.
* If the text changes or unkonwn, use *String*
* Rust uses *&str* type for string by default.

See sample code as follows:
```
fn main() {
    let uppercase_s = 'S';
    let lowercase_f: char = 'f';
    let smiley_face = '😃';
    let string_1 = "miley";
    let string_2: &str = "ace";
    println!("{} is a {}{} {}{}", 
        smilely_face, uppercase_s, 
        string_1, lowercase_f, string_2);
}
```

#### Tuples and Structs
Tuple elements can have different types.
Seperated with *,* and enclosed with *()*.
Tuple type is determined by all its elements.

Struct is a type composed of other types.
Struct fields can have different types.
Each field can have a meaningful name.

There are three kinds of Structs:
* Classic: most used by <struct>.<field>
* Tuple: fileds have no names <tuple>.<index>
* Unit: most commonly used as markers

```
fn main() {
    // define a classic struct
    struct Student { 
        name: String, 
        level: u8, 
        remote: bool
    }
    // define a tuple struct
    struct Grades (char, char, char, char, f32);

    let user_1 = Student {
        // convert string literal to String
        name: String::from("User One"),
        remote: true,
        level: 2
    };

    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);

    println!("{}, level {}, Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote,
        mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
}
```

#### Convert &str to String
Remember *&str* is the default type for "string".
Use `String::from("value")` to conver it to String.
If *expected struct `String`, found `&str`* happens.
For example, `name: String::from("User One")` above.

#### Enum - for compound data
An enum type can have any combination of variants.
Variants can have field with name or not, or no field.
Here is an example of enum to classify web event:
```
// can have a mixture of different variants
enum WebEvent {
    // like a unit struct without fields or data types
    WELoad,
    // like a tuple struct with type but no field name
    WEKeys(String, char),
    // like a classic struct with field and name
    WEClick { x: i64, y: i64 }
}
```
A fu that uses an enum must accept all its variants.
We can define an enum with structs as its variants.
Create enum variants with *::* and *let* keyword.
Add *#[derive(Debug)]* to data for println! output.
Use *{:#?}* to format data in readable manner. 
```
fn main() {
    // Define a tuple struct
    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
```
#### Function
Functions are defined with *fn* keyword.
Fns besides *main* can be defined anywhere.
Function name could not start with number.
Input args are in *()* with *,* seperator.
Function body is wrapped with *{}* syntax.
```
fn hello() {
    println!("Hello There!");
}

fn main() {
    hello();
    println!("100 divide by 3 is: {}", divide_by(100.0, 3.0));
    goodbye("Friend");
}

fn goodbye(name:&str) {
    println!("Goodbye {}!", name);
}

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
```

#### Program
Here is a complete sample program for demo functions:
```
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}

fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
}
```

