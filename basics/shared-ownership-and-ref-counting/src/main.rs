use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    //Static
    //static X: [i32; 3] = [1, 2, 3];
    //thread::spawn(|| dbg!(&X));
    //thread::spawn(|| dbg!(&X));

    //Leaking
    //let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    //thread::spawn(move || dbg!(x));
    //thread::spawn(move || dbg!(x));

    //Reference counting
    //let a = Rc::new([1, 2, 3]);
    //let b = a.clone();

    //This is not thread safe
    //thread::spawn(move || dbg!(b));
    //assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation

    //let a = Arc::new([1, 2, 3]);
    //let b = a.clone();

    //thread::spawn(move || dbg!(a));
    //thread::spawn(move || dbg!(b));

    //Shadowing recommended for reuse of var naming
    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        let a = a.clone();
        move || dbg!(a)
    });

    dbg!(a);
}
