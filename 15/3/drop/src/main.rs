struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };      // 俺のもの
    let d = CustomSmartPointer { data: String::from("other stuff") };   // 別のもの
    println!("CustomSmartPointers created.");                           // CustomSmartPointerが生成された

    // let c = CustomSmartPointer { data: String::from("some data") };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // // mainの終端の前にCustomSmartPointerがドロップされた
    // println!("CustomSmartPointer dropped before the end of main.");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    // CustomSmartPointerはmainが終わる前にドロップされた
    println!("CustomSmartPointer dropped before the end of main.");
}