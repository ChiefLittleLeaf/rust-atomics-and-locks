use std::thread;

fn main() {
    let nums = vec![1,2,3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", nums.len());
        });
        s.spawn(|| {
            for n in &nums {
            println!("{n}");
            }
        });
    });
}
