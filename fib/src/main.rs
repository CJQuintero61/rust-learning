/*
    computes the n-th fibonacci number
*/
fn main() {
    let n: i32 = 10;
    let recursive: i32 = fib_recursive(n);
    let iterative: i32 = fib_iterative(n);


    println!("the recursive result is {recursive}");
    println!("the iterative result is {iterative}");


}

fn fib_recursive(x: i32) -> i32 {
    if x <= 1 {
        return x
    }

    return fib_recursive(x - 1) + fib_recursive(x - 2)
}

fn fib_iterative(x: i32) -> i32 {
    if x <= 1 {
        return x
    }

    let mut temp: i32;
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    for _ in 2..x {
        temp = a;
        a = b;
        b = temp + b;
    }
    return b
}