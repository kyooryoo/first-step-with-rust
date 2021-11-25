use std::collections::HashMap;
use std::io::stdin;

#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }
    (Age::New, miles)
}

// create hash map elements with some routines
fn car_factory(order: u32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let color = colors[((order-1) % 4) as usize];

    let motor: Transmission;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    } else {                      // 1, 5, 7, 11
        motor = Transmission::Manual;
    }

    Car {
        color: color.to_string(),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    println!("For palying with Hash Map!");

    let mut car: Car;
    let mut orders: HashMap<u32, Car> = HashMap::new();

    println!("How many cars to create?");
    let mut num: String = String::new();
    stdin().read_line(&mut num).expect("Cannot read input!");
    // remove the extra new line in read line input value
    let num = num.trim().parse::<u32>().unwrap();

    for order in 0..num {
        let miles: u32 = (order % 4) * 700;
        let order = order + 1;
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));
    }
}