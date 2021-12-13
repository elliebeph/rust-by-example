// C-like enums.
#![allow(dead_code)]

// Enum with implicit discriminator (starts at 0).
enum Number {
    Zero,
    One,
    Two,
}

// Enum with explicit discriminator.
enum Colour {
    Red   = 0xff0000,
    Green = 0x00ff00,
    Blue  = 0x0000ff,
}

fn main() {
    // Enums can be cast as integers.
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Colour::Red as i32);
    println!("Violets are #{:06x}", Colour::Blue as i32);
}
