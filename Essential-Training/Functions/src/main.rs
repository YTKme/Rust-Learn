fn main() {
    println!("Function");

    say_hello();
    say_hello();

    let first_number: u8 = 10;
    let second_number: u8 = 20;
    say_the_sum(first_number, second_number);
    // say_a_number(second_number); // Error: expected `i32`, found `u8`

    let square_number = 7;
    println!("Square of {square_number} is: {:?}", square(square_number));

    let celsius_temperature: f32 = 37.0;
    let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);
    println!("Celsius: {celsius_temperature} C = Fahrenheit: {fahrenheit_temperature} F");
}

fn say_hello() {
    println!("Hello! From The `say_hello` Function!");

    say_a_number(5);
}

fn say_a_number(number: i32) {
    println!("Hello Number: {number}!");
}

fn say_the_sum(first: u8, second: u8) {
    let sum = first + second;
    println!("The sum of {first} and {second} is: {sum}!");
}

// Statement versus Expression
// Expression: a + b
// Statement: let sum = a + b;

// Function Return Value
fn square(number: i32) -> (i32, i32) {
    println!("Square: {number}");

    return (number, number * number);
}

// Temperature Conversion
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    // Formula: (C * 9/5) + 32
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    fahrenheit
}
