/*
    rust code is compiled using rustc file.rs
    which makes a binary executable just like gcc or clang.

    Note that all code I will be making will be compiled
    and ran in Rust installed within WSL.

    Rust is an ahead-of-time compiled lang - you can combine the binaries
    then give it to anyone and they can run it even if they don't have rust installed
*/

// standard comments

/*
    standard block comments
*/

fn main() {
    /*
        it is important to note that using ! after a function name,
        makes it a MACRO. Macros behave differently than regular functions.
    */
    println!("Hello World!");
    println!("I'm a Rustacean!");
}

//! generates docs for the enclosing item (typically used at the top of a file)

/// generates docs for the item that follows

// you can even write comments inside of expressions