fn main() {
    println!("Ownership");

    let planet = "Earth";
    if true {
        println!("The Planet: {planet}");
    }
    println!("The Planet: {planet}");

    let second_planet = "Earth";
    println!("The Second Planet: {second_planet}");
    let second_planet = "Mars";
    println!("The Second Planet: {second_planet}");

    println!("Shadow Variable");
    let third_planet = "Earth";
    {
        println!("The Third Planet: {third_planet}");
        let mut third_planet = 4;
        println!("The Mutated Third Planet: {third_planet}");
    }
    println!("The Third Planet: {third_planet}");

    println!("Stack and Heap");

    println!("String Data Type");

    let mut message = String::from("Earth");
    println!("The Message: {message}");
    message.push_str(" Is Home.");
    println!("The Message: {message}");

    println!("Ownership");

    println!("Move, Clone, and Copy");

    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("Inner Planet: {inner_planet}");
    }
    println!("Outer Planet: {outer_planet}");

    println!("Transfer Ownership");

    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("Rocket Fuel: {rocket_fuel}");
}

fn process_fuel(propellant: String) -> String {
    println!("Processing Propellant: {propellant}");
    let new_fuel = String::from("LNG");
    new_fuel
}
