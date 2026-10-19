use std::io; // input output library

use rand::prelude::*; // prelude module has the most commonly used items from the rand crate

use std::cmp::Ordering; // Ordering is an enum type that has three variants: Less, Greater, and Equal

fn main() {
    println!("Guess the number!");

    //rand::rng gives a RNG for the current thread of exec and is seeded by the OS
    // the random_range() takes a range param and is inclusive of both ends
    let secret_number = rand::rng().random_range(1..=100);

    // the loop keyword creates an infinite loop
    loop {
        println!("Please input your guess.");
        
        // rust vars by default are immutable and require mut to make them mutable
        let mut guess = String::new();

        // references are also immutable by default so we need to use &mut to make it mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // the compiler is smart enough to infer that this parse (which changes the type of a string)
        // is a u32 because we specified that the guess variable is a u32
        // also, it is smart enough to know we are comparing guess, a u32, to secret_number, so the
        // compiler infers that secret_number is also a u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // read line returns a Result value which is an enum type that can be either Ok or Err
        // if you dont call expect(), the code will compile but you'll get a warning

        println!("You guessed: {guess}");

        // to install dependencies, you add it to the cargo file

        // cargo will create a lock file to ensure every build uses the same version of the dependency
        // so that the output is always the same. It is important to commit the lock file to version control

        // you can run cargo doc --open to build documentation for your project and open it in a browser

        // matching matches your input value to the reference value and returns an Ordering enum value
        // which we call "arms". 

        // rust is strongly and statically typed but it also has type inference like we did on secret number and guess

        // by default rust numbers default to i32, a 32 bit signed int
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
