mod hust;
use crate::hust::Hust;

fn main() {
    let obj = Hust::new("Hi hust".to_string());
    obj.getter();
}
