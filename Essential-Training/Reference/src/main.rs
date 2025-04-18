fn main() {
    println!("Reference");

    println!("Borrowing Reference");

    // let rocket_fuel = String::from("RP-1");
    // let length = process_fuel(&rocket_fuel);
    // println!("Rocket Fuel: {rocket_fuel}, Length: {length}");

    println!("Mutable Reference");

    // let mut rocket_fuel = String::from("RP-2");
    // let length = process_fuel(&mut rocket_fuel);
    // println!("Rocket Fuel: {rocket_fuel}, Length: {length}");

    println!("Dangling Reference");

    // let rocket_fuel = produce_fuel();
    // println!("Rocket Fuel: {rocket_fuel}");

    println!("Slice");

    // let message = String::from("Greetings from Earth!");
    // println!("Message: {message}");

    // let last_word = &message[message.len() - 6..];
    // println!("Last Word: {last_word}");

    // let planet_list = [1, 2, 3, 4, 5, 6, 7, 8];
    // let inner_planet: &[i32] = &planet_list[..4];
    // println!("Inner Planets: {inner_planet:?}");

    println!("Slice Function Parameter");

    let message = String::from("Greetings from Earth!");
    println!("Message: {message}");

    let first_word = get_first_word(&message[10..]);
    println!("First Word: {first_word}");

    println!("Challenge Trim Space");

    let test1 = "We need more space.";
    assert_eq!(trim_space(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_space(&test2), "There's space in front.");

    let test3 = String::from("There's space in the rear. ");
    assert_eq!(trim_space(&test3[..]), "There's space in the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_space(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_space(test5), "");

    let test6 = "";
    assert_eq!(trim_space(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_space(test7), "🚀");

    println!("Challenge Trim Space Complete");
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("Processing Propellant: {propellant}");
    propellant.push_str(" Is Highly Flammable!");
    let length = propellant.len();
    length
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}

fn get_first_word(message: &str) -> &str {
    let bytes = message.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &message[..index]; // Found a Space!
        }
    }

    &message // No Space Found!
}

fn trim_space(message: &str) -> &str {
    let mut start_index = 0;
    // Find start index
    for (index, character) in message.chars().enumerate() {
        if character != ' ' {
            start_index = index;
            break;
        }
    }

    let mut end_index = 0;
    // Find end index
    for (index, character) in message.chars().rev().enumerate() {
        if character != ' ' {
            end_index = message.len() - index;
            break;
        }
    }

    &message[start_index..end_index] // Trimmed Message
}
