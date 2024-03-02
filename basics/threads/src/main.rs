use std::thread;

fn main() {
    let t1 = thread::spawn(spawn_thread_example);
    let t2 = thread::spawn(spawn_thread_example);

    println!("Hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();

    let nums = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = nums.len();
        let sum = nums.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}

fn spawn_thread_example() {
    println!("hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
