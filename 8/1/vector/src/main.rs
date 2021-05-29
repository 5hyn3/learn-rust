fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // vで作業をする
    } // <- vはここでスコープを抜け、解放される

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    let v = vec![1, 2, 3, 4, 5];

    // error!
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // error!
    // v.push(6);

    println!("The first element is: {}", first);
}
