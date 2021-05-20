enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

fn route(ip_type: IpAddr) {}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // compile error
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    println!("Hello, world!");
}
