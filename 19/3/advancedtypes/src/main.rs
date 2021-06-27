use std::fmt;
use std::io::Error;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // こんにちは
    let s1: &str = "Hello there!";
    // 調子はどう？
    let s2: &str = "How's it going?";
}

fn bar() -> ! {
    panic!("")
}
