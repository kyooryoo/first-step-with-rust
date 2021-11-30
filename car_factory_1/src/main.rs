// this program is for demo function

// declare Car struct with four arributes
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// declare enum for car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// build a car with input args of
// - color in string type
// - transmission type in enum type
// - convertible in boolean type
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}

fn main() {
    println!("Hello, car!");

    // declare a mutable car variable and reuse it for all three cars
    let mut car: Car;
    
    car = car_factory(String::from("Red"), Transmission::Manual, false);
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