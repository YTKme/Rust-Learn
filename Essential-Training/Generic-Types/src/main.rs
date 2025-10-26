use std::mem;

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * (self.width + self.height)
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn sum_box<T: std::ops::Add<Output = T>>(first: Box<T>, second: Box<T>) -> Box<T> {
    Box::new(*first + *second)
}

fn main() {
    /* Generic struct definitions */

    // let rect = Rectangle {
    //     width: 1.2,
    //     height: 3.4,
    // };
    // let rect = Rectangle {
    //     width: 1,
    //     height: 3.4,
    // };
    // println!("Rectangle: {:?}", rect);

    /* Generic method definitions */

    // let rect = Rectangle {
    //     width: 1u8,
    //     // height: 3u16,
    //     height: 3u8,
    // };
    // println!("Rectangle: {:?}", rect);

    // println!("Width: {}", rect.get_width());
    // println!("Perimeter: {}", rect.get_perimeter());

    /* Generic function definitions */

    // println!("Biggest: {}", get_biggest(1, 2));
    // println!("Biggest: {}", get_biggest(1.2, 2.3));

    /* Box data type */

    // let vehicle = Shuttle {
    //     name: String::from("Atlantis"),
    //     crew_size: 7,
    //     propellant: 835958.0,
    // };
    // println!("Vehicle Size On Stack: {}", mem::size_of_val(&vehicle));

    // let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    // println!("Boxed Vehicle Size On Stack: {} Byte", mem::size_of_val(&boxed_vehicle));
    // println!("Boxed Vehicle Size On Heap: {} Byte", mem::size_of_val(&*boxed_vehicle));

    // let unboxed_vehicle: Shuttle = *boxed_vehicle;
    // println!("Unboxed Vehicle Size On Stack: {} Byte", mem::size_of_val(&unboxed_vehicle));

    /* Challenge */
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_box(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_box(pi, e), 5.85987);

    println!("Tests Passed!");
}
