use std::thread;
use std::time::Duration;

fn main() {
    let handler = thread::spawn(|| {
        for i in 0..5 {
            println!("spawner thead {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("mainer thead {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}
