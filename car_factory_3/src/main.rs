#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, miles)
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    let quality = car_quality(miles);
    if quality.0.eq(&Age::Used) {
        if roof {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);  
        } else {
            println!("Prepare a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);  

        }
    } else {
        if roof {
            println!("Prepare a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);  
        } else {
            println!("Prepare a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);  
        }
    }

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    println!();
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}