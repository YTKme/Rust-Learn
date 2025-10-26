#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_point_y(point: Point) -> u8 {
    point.1
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
}

fn main() {
    /* Defining structs */
    // let mut vehicle = Shuttle {
    //     name: String::from("Endeavour"),
    //     crew_size: 7,
    //     propellant: 835958.0,
    // };

    // println!("Shuttle Name: {}", vehicle.name);
    // println!("Shuttle Crew Size: {}", vehicle.crew_size);
    // println!("Shuttle Propellant: {}", vehicle.propellant);

    // vehicle.name = String::from("Atlantis");

    // println!("Vehicle Is: {:?}", vehicle);

    /* Struct update syntax */
    // let mut vehicle = Shuttle {
    //     name: String::from("Endeavour"),
    //     crew_size: 7,
    //     propellant: 835958.0,
    // };

    // let vehicle2 = Shuttle {
    //     name: String::from("Discovery"),
    //     ..vehicle
    // };

    // let vehicle2 = Shuttle {
    //     ..vehicle.clone()
    // };

    // vehicle.crew_size = 6;

    // println!("Shuttle Name: {}", vehicle.name);
    // println!("Shuttle Crew Size: {}", vehicle.crew_size);
    // println!("Shuttle Propellant: {}", vehicle.propellant);

    // vehicle.name = String::from("Atlantis");

    // println!("Vehicle 1 Is: {:?}", vehicle);
    // println!("Vehicle 2 Is: {:?}", vehicle2);

    /* Struct methods */

    // let mut vehicle = Shuttle {
    //     name: String::from("Endeavour"),
    //     crew_size: 7,
    //     propellant: 0.0,
    // };

    // let vehicle_name = vehicle.get_name();
    // println!("Vehicle Name: {}", vehicle_name);

    // println!("Propellant Before: {}", vehicle.propellant);
    // vehicle.add_fuel(1000.0);
    // println!("Propellant After: {}", vehicle.propellant);

    /* Associated functions */

    // let mut vehicle = Shuttle::new("Endeavour");
    // let mut vehicle2 = Shuttle::new("Discovery");

    // let vehicle_name = vehicle.get_name();
    // println!("Vehicle Name: {}", vehicle_name);

    // println!("Propellant Before: {}", vehicle.propellant);
    // vehicle.add_fuel(1000.0);
    // println!("Propellant After: {}", vehicle.propellant);

    /* Tuple structs */
    // let red = Color(255, 0, 0);
    // println!("First Red Value: {}", red.0);

    // let coord = Point(4, 5, 6);
    // let y = get_point_y(coord);
    // println!("Y Value: {}", y);

    /* Challenge */
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Test Passed!");
}
