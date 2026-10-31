# References and Borrowing

the issue with passing in a string to a function
is that if we want to use that string after the function
we must return it back to the caller.

Instead we can give a reference to the string. A reference
can be followed to access data stored at that address
when that data is owned by some other variable.

references are guaranteed to point to a valid value of
a particular type for its lifetime. Pointers are not.

```rust
fn main() {
    let s1 = String::from("hello");

    // pass a reference to s1 as a param
    let len = calc_length(&s1);
}

fn calc_length(s: &String) -> usize {
    /*
        calculates the length of a string

        Args:
            s (&String) - a reference to a string
        
        Returns:
            usize - an unsized integer, architecture dependent
    */
    s.len()
}
```

when &s1 is passed, the value it points to will not be dropped from
memory when the reference is done being used since it does not own it.
And also the function uses &String to know the type of the parameter
is a reference. Passing references means you don't have to
give back ownership since ownership was never taken in the first place.

creating a reference is called borrowing. References are also
immutable by default and throw errors if you try to mutate them.

## Mutable References

to define a mutable reference

```rust
fn main() {
    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    /*
        some_string (&mut String) - a mutable reference to a string
    */
    some_string.push_str(", World!");
}
```

First the variable has to be declared as mutable, then a mutable reference
to that variable must be passed into the call, then the definition
of the function must have a mutable reference to a String type.

This makes it very clear that this function will cause a side effect
and will mutate the value it borrows.

NOTE: There is 1 big restriction on mutable references: if you have a
mutable reference to a value, you can't have any other references to that value.
This makes mutability very controlled. This also helps prevent data races at compile
time

Data races happen when 3 behaviors occur

- 2+ pointers access the same data at the same time
- at least 1 of the ptrs is being used to write to the data
- there's no mechanism being used to synch access to the data

Rust prevents data races by refusing to compile code with them

you can have as many immutable refs as you want but having
1 mut ref does not allow any other refs. Immutable refs
do not expect the ref to ever change so they cannot be
used with mutables.

You can however use them after their final use
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
// let r3 = mut &s; // BIG PROBLEM

println!("{r1} and {r2}");
// since r1 and r2 are no longer used, you can now
// define a mutable with no problem

let r3 = mut &s; // good
println!("{r3}");
```

this is allowed because the scopes of r1 and r2 end after
the println! since that is where they are last used, which is
before the r3 def so the scopes do not overlap.

The compiler can tell a ref is no longer being
used at a point before the end of the scope.

Remember that rust does this to prevent bugs and catch things at
compile time rather than at runtime

## Dangling References

```rust
// BAD
fn dangle() -> &String {
    let s = String::from("hello"); // s is a new string

    &s // return a ref to the String s
} // here, s goes out of scope and is dropped

// GOOD
fn no_dangle() -> String {
    let s = String::from("hello");

    s   // move ownership outside of this function, nothing is deallocated
}
```

## Recap

- you can have 1 mut ref, or many immut refs
- references must always be valid
