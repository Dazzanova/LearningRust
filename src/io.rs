use std::io::{self, Write};

pub fn run() {
    // trim removes whitespace from beginning and end, \n too
    // read_line: pass a mutable reference, allowing fn to modify the existing string without taking ownership
    // parse converts string into another type

    // string
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim();
    println!("Hello, {}!", name);

    io::stdout().flush().unwrap();

    // integer
    let mut age = String::new();

    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap();
    println!("You are {} years old.", age);

    io::stdout().flush().unwrap();

    let next_year_age = age + 1;
    println!("Next year, age is {next_year_age}");
}
