# Cargo

- the built system and package manager for rust
- manages dependencies for you

## Making a project with cargo

```shell
cargo new hello_cargo
cd hello_cargo
```

You can also turn existing projects into Cargo projects
with `cargo init` and moving your code to an src/ directory.
cargo init makes the cargo.toml file.

## Building and Running with Cargo

`cargo build` makes an executable file in target/debug/hello_cargo
rather than in current dir.

You can also run `cargo run` to compile and run in 1 command.
This is the common practice. When using cargo run, the compiler
can tell when code changes. If there were no changes,
then the binary is just ran, otherwise it compiles
then runs.

There is also `cargo check` which checks your code to make
sure it will compile but doesn't make an executable. This
is useful if you are constantly checking if your code
can compile. This is much faster than building each time.

## Building Releases

`cargo build --release` compiles with optimizations. This makes
the executable in target/release instead of target/debug. The optimizations
make the code much faster when running but take longer to compile. This is
for when you are ready to give the executable to someone else.