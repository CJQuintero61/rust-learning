
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    // rust vars are immutable by default
    x = 6;
    println!("The value of x is: {x}");

    const CONSTANT: u32 = 30;
    // constants are defined with the const keyword
    // and must declare a data type
    // they also cannot be assigned
    // by values that are determined at runtime

    // rust allows shadowing
    let x = 5;
    let x = x + 1; // no compile time error since it redefines the same variable name

    {
        let x = x * 2;
        println!("The value of x is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6


    // rust is statically typed meaning it must know the type
    // of all vars before runtime 

    // scalar types: ints, floats, bools, characters

    // integers can be signed or unsigned and 
    // can be 8 bit, 16, 32, 64, 128, or architecture dependent
    let myInt: isize = 12345; // arch dependent

    // there are standard floats

    // 5 basic arithmetic operations + - * / % 
    // integer division floors downward

    // standard bools
    let my_bool: bool = true;
    println!("The value of myBool is: {my_bool}");

    // standard char types in single quotes


    // rust has 2 compound types - tuples and arrays
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // the tup var binds to the entire tuple
    let tup = (500, 6.4, 1);

    // tuple unpacking
    let (x, y, z) = tup;

    // array
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // init an array full of 3s with length 5
    let my_array = [3; 5] // init value; length
    // index arrays with array[x] notation

    // NOTE: Rust has index checking on arrays so it will
    // panick and stop exec during runtime when you try to
    // access an item out of bounds
}