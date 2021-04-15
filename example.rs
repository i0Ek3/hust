// reference standard libaray
use std::f64::consts::PI;

// import module
mod hust_mod;

// enable debug mode
#[derive(Debug)]

// define struct
struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

fn h(hi: String) {
    println!("{}", hi);
}

// use keyword for module
use crate::hust_mod::hust::_1::_1f;
use crate::hust_mod::hust::_1::_1f as f1;

fn main() {
    let hi = "Hi rust";
    let _i: i32 = 0;
    for _i in 0..7 {
        h(hi.to_string());
    }

    let str = String::from(hi);
    let slice = &str[..2];
    println!("{}", slice);

    let t = Triangle{ a: 10, b: 20, c: 30 };
    println!("t is {:?}", t);

    //println!("{}", hust_mod::hust::_1::_1f())
    println!("{}", _1f());
    println!("{}", f1());

    println!("{}", (PI * 4.0 / 3.0).cos());
}
