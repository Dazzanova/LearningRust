fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

//note: no semicolon here

fn sqaure_num(n: i32) -> i32 {
    n * n
}

fn is_even(n: i32) -> bool {
    n % 2 == 0;
}

pub fn run() {
    greet("Max");

    let result = add(69, 4);
    println!("res : {result}");

    let squared = sqaure_num(4);
    println!("squared : {squared}");

    let even = is_even(4);
    println!("even : {even}");
}
