fn main() {
    // try_loop()
    // try_while()
    // iterate()
    range()
}

fn try_loop() {
    loop {
        println!("again!"); // また
    }
}

fn try_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // 発射！
    println!("LIFTOFF!!!");
}

fn iterate() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}

fn range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
