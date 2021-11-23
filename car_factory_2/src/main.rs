fn main() {
    println!("Hello, Cars!");

    let colors = [ "Blue", "Green", "Red", "Silver" ];

    let mut car: Car;
    car = car_factory(colors[0].to_string(), Transmission::Manual, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1 );
    car = car_factory(colors[1].to_string(), Transmission::SemiAuto, false, 100);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1 );
    car = car_factory(colors[2].to_string(), Transmission::Automatic, true, 200);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1 );

}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        (Age::Used, miles)
    } else {
        (Age::New, miles)
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color,
        motor,
        roof,
        age: car_quality(miles),
    }
}
