fn main() {
    println!("Array");

    let letter_array = ['a', 'b', 'c'];
    println!("Letter Array: {:?}", letter_array);

    let first_letter = letter_array[0];
    println!("The first letter is: {first_letter}");

    // Mutable Array
    let mut number_array = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", number_array);

    number_array[1] = 10;
    let second_number = number_array[1];
    println!("The second number is: {second_number}");

    // Uninitialized Array
    let empty_number_array: [i32; 5];
    // empty_number_array = [0, 0, 0, 0, 0];
    empty_number_array = [0; 5]; // Repeat expression
    let index: usize = empty_number_array.len() - 1;
    println!("Empty Number Array Index: {:?}", index);

    // Multidimensional Array (2D Array)
    let parking_lot = [
        [1, 2, 3], // Row 0
        [4, 5, 6], // Row 1
        [7, 8, 9], // Row 2
    ];
    let parking_spot = parking_lot[1][2];
    println!("Parking Lot: {:?}", parking_lot);
    println!("Parking Spot: {parking_spot}");

    // Multidimensional Array (3D Array)
    // let garage = [[[0; 100]; 20]; 5];
    let garage = [[[0; 10]; 3]; 5];
    println!("Garage: {:?}", garage);

    println!("Tuple");

    // Tuple Basic
    let stuff_tuple: (u8, f32, char) = (10, 3.14, 'x');
    println!("Stuff Tuple: {:?}", stuff_tuple);
    let first_stuff = stuff_tuple.0;
    println!("First Stuff: {first_stuff}");

    // Mutable Tuple
    let mut mutable_tuple: (u8, f32, char) = (10, 3.14, 'x');
    println!("Mutable Tuple: {:?}", mutable_tuple);
    mutable_tuple.0 += 20;
    println!("Mutable Tuple: {:?}", mutable_tuple);

    // Unpack Tuple
    let (first_stuff, second_stuff, third_stuff) = stuff_tuple;
    println!("Unpacked Stuff Tuple: First: {first_stuff}, Second: {second_stuff}, Third: {third_stuff}");
}
