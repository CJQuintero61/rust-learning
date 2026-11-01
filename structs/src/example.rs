// stucts examples
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 3,
    };

    println!(
        "The area of the rectangle is {}",
        area(&rect1)
    );

    // printing a struct directly won't work
    // so we need to specify how we want to print it
    // here, the :? specifier means print the output
    // in Debug format
    println!("rect1 is {rect1:?}");

    // you can also use :#? to make the output prettier

    // there is also the dbg! macro which prints to stderr
    // and takes ownership while println! takes a ref and prints to stdout
}

fn area(rectangle: &Rectangle) -> u32 {
    // return area
    rectangle.width * rectangle.height
}