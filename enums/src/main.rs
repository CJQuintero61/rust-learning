// enums are a way of saying a value is 1 of a
// possible set of values
enum IpAddrKind {
    // you can store data with each enum variant
    // directly inside of the definition
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // also note that the enum name also is a function
    // that constructs an instance of that enum
    // so IpAddr::V4() takes in a String and returns an IpAddr type
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // we can also have enum variants with different associated data
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);

    // enums can take other params too
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // named fields
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // you can also write methods in a impl block
    // for enums just like structs

    // there is also the Option<T> enum provided
    // by the std library for a value existing
    // or being none
    let some_number = Some(5);
    let some_char = Some('e');

    // we must annotate for the compiler this None is an i32 type\
    let empty_number: Option<i32> = None;

    // also, T and Option<T> are not the same
    // ERROR
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;

    //  if you ever expect the possibility of an empty value,
    // you must explicitly opt in by making the type of that value Option<T>
    // then you are required to handle the case where the value is null

    // everywhere else where you use a type that isn't Option<T>, you can
    // safely assume a value will be present
}
