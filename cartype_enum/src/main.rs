#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault
}

fn car_nationality(car:CarType){
    match car{
        CarType::Fiat => println!("The car is italian!"),
        CarType::Ford => println!("The car is american!"),
        CarType::Renault => println!("The car is french")
    }
}

fn main() {
    car_nationality(CarType::Fiat);
    car_nationality(CarType::Ford);
    car_nationality(CarType::Renault);
}
