fn main() {
    // Declaring Variable
    println!("# Declaring Variable");
    let mut x = 10;
    println!("The value of x is: {}", x);
    x = 20;
    println!("The new value of x is: {}", x);

    // Integer Data Type
    println!("# Integer Data Type");
    // let mut x: u8 = 255; // Overflow
    let mut y: u8 = 254;
    y = y + 1;
    println!("The value of y is: {}", y);

    // Floating Point Data Type
    println!("# Floating Point Data Type");
    let z: f32 = 10.123456789123456789;
    println!("The value of z is: {}", z);

    // Arithmetic Operation
    println!("# Arithmetic Operation");
    // let a = 10;
    let a = 10.0;
    let b = 3;
    // let b = 3.0;
    // let c = a + b;
    // let c = a - b;
    // let c = a * b;
    // Casting
    let c = a / (b as f64 + 1.0);
    // let c = a % b;
    println!("The value of c is: {}", c);

    // Formatting Print Statement
    println!("# Formatting Print Statement");
    let d = 10.0;
    let e = 3.0;
    let f = d / e;
    println!("The value of f is: {:.2}", f);
    println!("The value of f is: {:08.2}\nThe value of d is: {}", f, d);

    // Bitwise Operation
    println!("# Bitwise Operation");
    let mut value = 0b1111_0101u8;
    println!("The value of value is: {}", value);
    println!("The value of value is: {:08b}", value);

    value = !value;
    println!("The new NOT value of value is: {:08b}", value);

    value = value & 0b1111_0111;
    println!("The new Bitwise AND value of value is: {:08b}", value);
    println!("The bit 6 of value is: {}", value & 0b0100_0000);

    value = value | 0b0010_0000;
    println!("The new Bitwise OR value of value is: {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("The new Bitwise XOR value of value is: {:08b}", value);

    value = value << 4;
    println!("The new Bitwise Left Shift value of value is: {:08b}", value);

    value = value >> 2;
    println!("The new Bitwise Right Shift value of value is: {:08b}", value);

    // Boolean Data Type and Operation
    println!("# Boolean Data Type and Operation");

    let g = true;
    let h = false;
    println!("The value of g is {}, h is {}", g, h);
    println!("The value of NOT g is {}", !g);
    println!("The value of g AND h is {}", g & h);
    println!("The value of g OR h is {}", g | h);
    println!("The value of g XOR h is {}", g ^ h);

    println!("Variable Average");

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test Passed!")
}
