fn main() {
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
