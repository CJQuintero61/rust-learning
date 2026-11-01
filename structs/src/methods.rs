// methods are functions that act on some object
// and take the &self parameter

struct Rectangle {
    w: u32,
    h: u32,
}

/*
    everything inside of an impl block will be associated
    with that type.

    All functions here will require the self first param
*/
impl Rectangle {
    // &self is shorthand for self: &Self
    // methods can take ownership, borrow, or take mutable borrwing
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    // associated functions are associated with the type
    // they are defined in and do not take a self
    // param. Associated functions that aren't methods
    // are often used as constructors that return a new instance
    // of the struct. This is like calling 'new' in other langs
    // here, Self is an alias for Rectangle
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }

    // each struct can also have multiple impl blocks
    // but it is usually better to just group it all
    // under 1 block
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // rect1.can_hold(&r2); // immutable borrow to read r2's data without taking ownership
}