use std::thread;

fn main() {
    //Static
    //static X: [i32; 3] = [1, 2, 3];
    //thread::spawn(|| dbg!(&X));
    //thread::spawn(|| dbg!(&X));

    //Leaking
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}
