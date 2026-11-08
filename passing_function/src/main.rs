fn main() {
    // closures are anon functions that you can store
    // in a variable with params between pipes
    // closures also use type inference, so
    // type annotating the params is optional
    // let add_one = |x: i32| x + 1;
    // println!("{}", add_one(5)); // 6

    funky()

    // closures can also be explicit on their return and param types
    // and can span multiple lines too like a function,
    // but stored in a var
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // takes in a param and just returns it
    // the compiler infers the types of the params and return
    // calling this once, will bind the closure to a specific type
    let closure_type = |x| x;

    // binds closure type to only work on strings
    let s = closure_type(String::from("Hello"));

    // compiler error sicne the closure now expects string types
    let n = closure_type(n);

}


fn funky() {
    // the actual closure is the add_offset line
    // and it is able to see the offset var even
    // though it wasn't passed into it
    // normal functions can't do this
    let offset: i32 = 10;
    let add_offset = |x: i32| x + offset;
    println!("{}", add_offset(5));
}