
/*
    the notation <F: Fn(i32) -> i32> means F is any function
    or closure that takes in an i32 param and returns an i32

    then f: F is the local param name of type F defined in the <> before it
*/
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn add_three(x: i32) -> i32 {
    x + 3
}

fn main() {
    // passing a regular function
    let a = apply_twice(add_three, 10);
    println!("{a}"); // 16

    // passing a closure
    let b = apply_twice(|x| x * 2, 10);
    println!("{b}"); // 40

    

}

// you can also use a function pointer instead of a generic
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}