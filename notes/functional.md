# Functional Concepts

## Closures

anonymous functions you can save in a var or pass as args
to other functions. You can define it anywhere then
call it anywhere. They can capture values
from the scope in which they are defined, which functions
cannot do.

Closures, just like functions, can take a a param in 3 ways

- borrow immutably
- borrow mutably
- take ownership

The closure decided which of these to use based on
what the body of the function does with its captured values

```rust
// example of an immutable borrow
// so the list is a valid param
// before and after the closure is called
// remember that a var can have many immutable references
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

```rust
// example of a mutable borrow
// the closure takes in a mutable reference to the list
// mutates it then ends. The ownership is still
// with list in main
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // HERE: it is important to note that the closure def above
    // creates a mutable borrow to the list variable, which can only
    // have 1 mut ref. because of this, we cannot make an immutable borrow
    // to print between the closure def and call.
    borrows_mutably();
    // now can make immutable refs
    println!("After calling closure: {list:?}");
}
```

```rust
// closures can also take ownership by using the move keyword
// this is maily useful for passing a closure to a new thread
// to move the data, so that the data is now owned by the new thread
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

In the above ex, it is also important to note that the compiler will throw an error
if you don't use move here since then the closure takes the immutable ref
of the list, but can be invalid if the main thread finishes first. To keep
the ref valid, the compiler will error for this to keep all immutable
refs valid.


After getting values from passing things into the closure,
the body now defines what happens to references or values whent he closure is
evaluated later, affecting what is moved out of it

A closure body can do any of

- move a captured value out of the closer
- mutate a captured value
- neither move nor mutate a value
- capture nothing from the environment

The way a closure captures and handles values from the env affects which
triats the closure implements, and trains are how functions
and structs can specify what kinsd of closures they can use.

Closures will auto implement 1, 2, or 3 of these Fn traits
in an additive fashion depending on how the boyd handles the values

FnOnce - applies to closures that can be called once. All closures implement at least this
trait. A closure that moves captured values out of its body will only implemnt
FnOnce and no other traits since it can only be called once.

FnMut - applies to clsoures that don't move captured values out of their body but might
mutate the captured values. These closures can be called more than once.

Fn - applies to closures that don't move captured values out of their boyd
and don't mutate captured values, as well as closures that capture nothing
from their env. THese can be called many times without mutating
their env which is important in concurrency.

When a closure is defined, the ownership is moved into the closure automatically
based on what it does with its captures. Remember it can be moved, immut borrow, or mut 
borrow. The body determines the capture mode and the trait, but the capture
mode can be overridden with `move` even if the closure doens't need ownerhsip.

Thus, when a closure is defined, (not called), that is when it receives ownerhsip of
a variable and that ownerhsip can only be given once in the example.