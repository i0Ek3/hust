fn h(hi: String) {
    println!("{}", hi);
}

fn main() {
    let hi = "Hi rust";
    let i: i32 = 0;
    for i in 0..7 {
        h(hi.to_string());
    }
}
