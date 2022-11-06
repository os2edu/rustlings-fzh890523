// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data); // moved
    // can not use data now
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();  // to_uppercase(&self) can auto ref, but return String
    
    println!("{}", data);
}

fn string_uppercase1(data: &String) {
    //                         - let's call the lifetime of this reference `'1`
    // data: mut &String
    // data = &(data.to_uppercase());
    // 暂时不太清楚为什么下面可以而上面不可以，报错 temporary value dropped while borrowed
    // temporary value is freed at the end of this statement、creates a temporary which is freed while still in use
    // assignment requires that borrow lasts for `'1`
    // 好像与 mut & 的lifetime有关
    let data1 = &(data.to_lowercase());
    
    println!("{}", data);
    println!("{}", data1);
}
