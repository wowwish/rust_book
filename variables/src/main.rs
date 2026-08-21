// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x = 5;
    let x = x + 1; /*
    Shadowing within the same scope (main). This is different from
    modifying a mutable variable, as this will not work without the 'let' keyword.
    Also, in shadowing, the variable type can be modified which is not possible
    with a mutable variable modification
    */
    {
        // Shadowing starts for new scope (newscope)
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // Shadowing ends when the new scope is closed (newscope)
    println!("The value of x is: {x}");

    let _z = 3; // Prefix unused variables with an underscore to prevent compilation warnings
    // Such unused variables are only used to show concepts of rust and it is a bad practice
    // to leave such variables in your code!
}