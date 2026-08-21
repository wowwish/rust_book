fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("Themeasurement is: {value}{unit_label}");
}

fn main() {
    println!("Hello, world!");

    // Calling external functions
    another_function(5);
    print_labelled_measurement(5, 'h');

    /*
    In Rust, a block of code can also behave as an expression that returns a value
    Note that the last line in the block does not have an ending semicolon. This
    means that line is not an expression (as opposed to a statement does not return
    anything in rust). Any literal, function calls or macro calls
    (like println! calls) are all expressions in Rust with a return value.
    Note: adding a semicolon to the end of an expression converts it into a statement
    and you cannot capture it's return value - will error out.
    */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// A function with a return value - the type of the return value is
// mentioned in function definition with an arrow.
fn five() -> i32 {
    5 // remember that literals are expressions and the lack of semicolon here
    // means this function returns 5 as value
}