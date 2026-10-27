# Ownership

Rusts most unique feature. Ownership allows
the lang to be memory safe without needing a garbage collector.

Ownership is a set of rules that govern how a rust program manages memory.
The compiler checks these rules and if any are violated, the code
fails to compile.

## Review stack and heaps

the stack and heap is memory available to your code at runtime.

The stack stores values in a LIFO order (as a stack). All data stored on
stack must have a known fixed size. All data with unknown sizes at compile
time or a size that might change are stored on the heap instead.

The heap is less organized. When you request a certain amount of space, the memory allocator
finds an empty spot big enough, marks it in use, then returns a pointer to that location.

the pointer to the heap has a known fixed size after allocating it, so then
you can store the pointer on the stack, but when you want the actual data,
you must follow the pointer.

Pushing onto the stack is faster than allocating since the allocator doesn't have to search
for a place to store new data; the location is always at the top of the stack. The allocator
first has to find a spot big enough to hold the data, then perform bookkeeping to prepare
for the next allocation.

Accessing data in the heap is also generally slower than accessing data on the stack because
you have to follow a pointer to get there. Since stack memory is all close to each other,
it is much faster than jumping between addresses.

When a function is called, the values passed to the function and the function's local
vars get pushed onto the stack, then popped off when the function ends.

## back to ownership now

rules

- each value in rust has an owner
- there can only be 1 owner at a time
- when the owner goes out of scope, the value will be dropped

## the String type

the other primitives from Ch3 all have a fixed known size and copying them
is easy and quick and they are all stack vars.

```rust
// string literals are immutable
let s: String = "hello string literal";

// non-literal strings can be mutated
let mut st = String::from("hello");
s.push_str(", world!"); // append to the end of the string
println!("{s}");
```

the reason string literals can't be mutated but others can is because
of how the 2 types deal with memory.

## Memory Allocation

literals are fast and efficient and we know the contents at compile time.
These benefits only happen becuase the string literal is immutable.

non-literals have to: 1) request memory from the memory allocator at runtime,
and 2) return that memory to the allocator when we are done with it.

The first part is done by us when we call `String::from`, which requests
memory it needs.

In rust, the way automatic memory deallocation is done is through scope.
When a variable goes out of scope, rust calls a special function for us
called `drop` and that is where the author of `String` puts the code
to return the memory. Rust calls `drop` automatically at the
closing curly brace.

## Variables and Data Interacting with Move

normally

```rust
let x = 5;
let y = x;

// 2 vars that both hold 5 (stack vars) that are independent
```

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Internally, a string is made of 3 parts: a pointer to the contents, a length,
and a capacity. The 3 vars (pointer, length, cap) all are on the stack.
The heap memory holds the actual contents of the string and is what
the pointer points to.

The length is how much memory in bytes the contents of `String` are currently using.
The capacity is the total amount of memory, in bytes, that the String received
from the allocator.

When the assignment above is done, we copy the pointer, length,and capacity
that are on the stack. We do not copy the data on the heap that the pointer
refers to.

If rust actually copied heap data as well, we would have 2 independent string
copies that point to different locations but with the same contents,
but this doesn't happen and also copying is very expensive in performance
when the data on the heap is large

We know that rust calls the drop function at the ending curly brace, but
here, 2 vars both are trying to free the same memory, which is a double free error
and leads to memory corruption and security vulnerabilities.

To ensure memory safety, after `let s2 = s1;`, rust considers s1 as no longer valid.
Therefore, rust doesn't need to free anything when s1 goes out of scope.

If you try to reuse s1 after it is considered invalid (aka Use after free),
youll get an error since rust will prevent you from using invalid references.

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
```

This is similar to a shallow copy but since rust also invalidates the first variable,
this is called a move. We say s1 was moved into s2.

This solves our problem since now only s2 is valid and is in charge of freeing the memory.
Also, rust will never automatically create deep copies of your data. Therefore,
any automatic copying can assumed to be inexpensive in terms of runtime performance

Also, when you assign a completely new value to an existing variable, rust
will call drop and free the original value's memory immediately

```rust
let mut s = String::from("Hello");
s = String::from("ahoy");
println!("{s}");
```
since the original string hello is no longer referred to on the heap,
it goes out of scope, so drop is called and frees it.

If you do want deep copies, there is the clone method
```rust
let s1 = String::from("Hello");
let s2 = s1.clone(); // deep copy; computationally expensive
```

the integer example doesn't need clone since copying from the stack
is quick and there is no reason we would want to prevent x from being
valid after we create y.

Rust also has a special annotation called the `Copy` trait that can be placed
on types that are stored on the stack. If a type implements the Copy trait,
variables that use it do not move but are trivially copied making them valid
after assignment to another var.

see the documentation for specific on what implements Copy, but generally
all primitives have it and tuples if they only have types that also
implement Copy.

## Ownership and Functions

When you pass a reference into a function, the function parameter now
is the valid owner of that heap memory, so the parent functions value
is no longer valid

```rust
// inside of main
let s = String::from("Hello");

my_function(s);
// s's ownership was transfered to the variable in the function
// so s at this point is no longer valid and the local param
// in the function is also no longer valid.
```

This makes the language predictable: assigning a value to another var moves it,
and when a var that includes heap data goes out of scope it is cleaned up
unless ownership is transfered to another var.

But what if I want to use a value in a function but not give up ownership of it?
1. you can either return a tuple from the function to get the ownership back after the function returns, or

2. rust has a feature for using values without transfering ownership: references!