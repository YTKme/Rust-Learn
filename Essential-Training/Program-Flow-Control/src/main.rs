fn main() {
    println!("Program Flow Control");

    // Conditional Execution
    // let x = 3;

    // if x == 3 {
    //     println!("X: {x}");
    // }

    // Multiple Condition
    let x = 3;
    let y = 5;

    if x > y {
        println!("X is greater than Y");
    } else if x < y {
        println!("X is less than Y");
    } else {
        println!("X is equal to Y");
    }

    // Conditional Assignment
    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    }

    println!("X: {x}");

    // Loop
    println!("Loop");
    let mut count = 0;

    let result = loop {
        count += 1;
        println!("Count: {count}");

        if count == 5 {
            break count;
        }
    };

    println!("After Loop");
    println!("Result: {result}");

    // while Loop
    println!("While Loop");
    let mut count = 0;
    let letter_list = ['a', 'b', 'c', 'd', 'e'];

    while count < 10 {
        count += 1;
        println!("Count: {count}");

        if count < 5 {
            println!("Letter: {}", letter_list[count-1]);
        }
    }

    // For Loop
    println!("For Loop");
    let letter_list = ['h', 'e', 'l', 'l', 'o'];

    for letter in letter_list {
        println!("Letter: {letter}");
    }

    for (index, letter) in letter_list.iter().enumerate() {
        println!("Index: {index}, Letter: {letter}");
    }

    for number in 0..5 {
        println!("Number: {number}");
    }

    // Nested Loop
    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    for row in matrix.iter() {
        for num in row.iter() {
            println!("Number: {num}");
        }
    }

    for row in matrix.iter_mut() {
        for number in row.iter_mut() {
            *number *= 10;
            print!("Number: {number}\t");
        }
        println!();
    }

}
