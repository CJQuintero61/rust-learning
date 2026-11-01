fn main() {
    // more flexible than tuples since they name their values

    // define the struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // instance
    let user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("user@email.com"),
        sign_in_count: 1,
    };

    // to mutate an instance, the entire instance must be mutable
    // and you use dot notation to access

    // if you want to make another instance with the same
    // values as another but only update some fields,
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // for all fields not specified, use user1's values
        // this has to be last in the order
    };

    // note that doing the above, moves the data so
    // so the username now is owned by user2

    // structs also have tuple structs which
    // help cut down on syntax for simple structs
    struct Color(i32, i32, i32);

    // there is also unit types which is just to declare a type
    // without providing data
    struct AlwaysEqual;

    // a big note is that notice we use the owned String type
    // and not &str string slice type. This is because
    // we want each struct to own all of its data
}

fn build_user(emai: String, username: String) -> User {
    // method to create users

    // you could write out username: username, or use the shorthand
    // since they both have the same name
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
