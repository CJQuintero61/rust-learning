# Rust Notes

## Types

Rust is Statically typed and must know the type of all vars during
compile time.

Rust has type inference but you can also specify the type you want

follows snake case for regular vars and ALL_CAPS_WITH_UNDERSCORES for constants

constants and immutables are slightly different

### Primitives

- integers (different bit lengths and signed/unsigned)
- floats
- bools
- chars

### Compound types

- tuples (support tuple unpacking and can specify the type of each item in a tuple)
  - items can be accessed by using a dot `my_tuple.0` for the first item
- Arrays
  - all items must be the same
  - fixed size
  - lives on the stack like all other primitives
    - we will learn vectors later
  - rust will panick and crash when you try to access an index out of bounds

