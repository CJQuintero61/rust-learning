fn main() {
    my_function(5);

    // statements and expressions
    // statements do not return anything and expressions do
    // return something and can be evaluated
    let y: i32 = {
        let x = 6;
        x + 1 // note that expression do not end in a semicolon
    }; // y is equal to 7
    println!("the value of y is {y}");

    let return_value = funky();

    // rust does not auto cast variables to bools when used in conditions
    let number = 7;
    if number != 0 {
        println!("true");
    }

    let value = if true {5} else {10};
    println!("the value of value is {value}");

    // mis matched types
    // the if and else blocks need to have the same type
    // so here the else block throws an error since its not the same
    // type as the if block
    // let value = if true {5} else {"six"};


    let twenty = {
        let mut counter = 1;
        loop {
            counter += 1;

            if counter == 10 {
                break counter * 2
            }
        }
    };
    // standard while loops

    // for loops
    let items: [i32; 5] = [1, 2, 3, 4, 5];
    for item in items {
        println!("the value is {item}");
    }

    for element in (1..4).rev() {
        println!("element is {element}");
    }
}

// must specify the type in param list
fn my_function(x: i32) {
    println!("the value of x is {x}");
} 

fn funky() -> i32 {
    6 + 7
}

/*
    rust does have multi line comments
    but the book suggests using // on every line
*/

