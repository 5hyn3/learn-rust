fn main() {
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        // '{}'の長さは、{}です
        println!("The length of '{}' is {}.", s1, len);
    }
    {
        let mut s = String::from("hello");

        change(&mut s);
    }
    {
        // error[E0499]: cannot borrow `s` as mutable more than once at a time

        // let mut s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

        let r2 = &mut s;
    }
    {
        let mut s = String::from("hello");

        let r1 = &s; // 問題なし
        let r2 = &s; // 問題なし
        let r3 = &mut s; // 大問題！
        println!("r3: {}", r3);
    }
    {
        // error[E0106]: missing lifetime specifier
        
        // let reference_to_nothing = dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
