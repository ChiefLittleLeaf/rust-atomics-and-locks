fn main() {
    let a = 10;
    let mut b = 0;
    f(&a, &mut b);

    // This section is not safe to do if the assumptions of index can
    // never be anything other than 1,2,3 are not upheld therefore the
    // last arm match could cause undefined behavior long before we get
    // to the unsafe code.
    let index = 0;
    match index {
        0 => println!("{index}"),
        1 => println!("{index}"),
        _ => println!("{index}"),
    }
    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(index) };
}

// Example function where the compiler knows a will never be changed so
// therefor we can remove the if statement as an optimization as it will never be executed.
fn f(a: &i32, b: &mut i32) {
    let before = *a;

    *b += 1;
    let after = *a;
    if before != after {
        println!("before is not equal to after"); // this is never called
    }
}
