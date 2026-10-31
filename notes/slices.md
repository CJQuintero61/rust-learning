# Slices

Slices let you ref a contiguous sequence of elements in a collection.
A slice is a kind of reference, so it doesn't have ownership.

Suppose we wanted to return part of a string
```rust
let mut s = String::from("hello world");

// you can optionally put the starting and ending
// where it is assumed to be the beginning or end
// if not specified
let hello = &s[..5];
let world = &s[6..];

// from start to end
let whole = &s[..];

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..];
}

// rust also checks for invalid references
let word = first_word(&s); // immutable borrow happens here

s.clear(); // error, since word is an immutable reference and s was just mutated
// and the immutable reference is later used
println!("first word is {word}");
```

string literals are immutable so &str is an immutable reference
```rust
// that means &String and &str are different
// &str is a string literal slice
// &String is a string reference

// using &str for the param here makes it work on
// &String and &str values making it more general
fn first_word(s: &str) -> &str {
    ...
}

// string literals *are* string slices already
// and by accepting a slice, &str, you can work
// on dynamic Strings and slices 

// slices are references to part of a string
```

you can also use slices for arrays