#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, 0)
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Prepare a Used car: {:?}, {}, hard top, {} miles", motor, color, miles);
        } else {
            println!("Prepare a Used car: {:?}, {}, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Prepare a New car: {:?}, {}, hard top, {} miles", motor, color, miles);
        } else {
            println!("Prepare a New car: {:?}, {}, {} miles", motor, color, miles);
        }
    }

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut engine = Transmission::Manual;
    car_factory(String::from(colors[0]), engine, true, 0);
    engine = Transmission::SemiAuto;
    car_factory(String::from(colors[1]), engine, false, 100);
    engine = Transmission::Automatic;
    car_factory(String::from(colors[2]), engine, true, 200);
}