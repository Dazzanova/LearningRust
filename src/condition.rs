pub fn run() {
    let age = 20;

    // simple if - else
    if age >= 18 {
        println!("Adult");
    } else {
        println!("Minor");
    }

    // using if
    let num = 10;

    let res = if num % 2 == 0 { "even" } else { "odd" };
    println!("The number {num} is {res}");

    // match
    let res2 = match num % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("The number {num} is {res2}");

    // match with multiple
    let ch = 'a';
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
        _ => println!("Consonant"),
    }
}
