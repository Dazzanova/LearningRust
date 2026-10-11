pub fn run() {
    let x = 60;
    // x += 10;    #throws error

    let mut z = x;
    z += 10;    // mut keyword -> allows to modify the value

    println!("x: {}", x);
    println!("z: {z}");

    // var types
    let age: u8 = 19;
    let temperature: f64 = 36.5;
    let active: bool = true;
    let grade: char = 'A';

    // Constants
    const MAXX: i32 = 100;
    println!("maxx: {MAXX}");

    let mut y = "hello".to_string();
    y += " world";
    println!("{}", y);
}
