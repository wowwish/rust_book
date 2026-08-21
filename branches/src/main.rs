fn main() {
    let number = 3;
    
    /*
    The if condition should always be an expression evaluating to a bool type in Rust
    Otherwise, we'll get an error during compilation. Rust will not automatically
    try implicit type conversion of the condition expression evaluation value to bool.
    Blocks of code associated with the conditions in if expressions are also referred to
    as "arms".
    */
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Handling multiple conditions with if-else-if statements
    let num = 6;

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // Using if in a let statement - both the if and else arms should return the same type
    // of value to prevent compilation error
    let condition = true;
    // let x = if condition { 5 } else { "six" }; // will throw error
    let x = if condition { 5 } else { 6 };
    println!("The value of x is: {x}");
}
