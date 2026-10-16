pub fn run() {
    // loop
    let mut count = 0;

    loop {
        println!("Count: {count}");
        count += 1;

        if count == 5 {
            break;
        }
    }

    // while
    let mut n = 5;

    while n > 0 {
        println!("n = {n}");
        n -= 1;
    }

    //for loop
    // IMP POINT: 1..5 -> 1, 2, 3, 4 while 1..=5 -> 1, 2, 3, 4, 5
    for i in 1..=5 {
        println!("i = {i}");
    }

    // loop returning a val
    // IMP POINT: at the end , put semi colon
    let mut temp = 1;

    let res = loop {
        temp *= 2;

        if temp >= 20 {
            break temp;
        }
    };

    println!("Result: {res}");

    // continue and break as it is, break returns a val

    let nums = [10, 20, 30];
    for num in nums {
        println!("num = {num}");
    }

    println!("Result: {res}");
}
