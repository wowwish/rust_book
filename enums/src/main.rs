// In Rust, Enums give you a way of saying that a value can be one of a set of possible
// values. Each possible value is called a Variant. Enums are custom data types like Structs
// in Rust.
enum IpAddrKind {
    V4,
    V6
}

// Instead of attaching the enum type as a field in a Struct, we can directly attach
// data to the variations of the Enum.
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
We can use an Enum instead of multiple Structs for each possible value of a type
The following set of Structs can be converted into a single Enum:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
};
struct WriteMessage(String); // tuple struct
struct ChangeColourMessage(i32, i32, i32); // tuple struct

We can define a single Enum with all these Struct types as variants. We can also
add methods to the Enum using an implementation block and these methods can take
any of the variants of the Enum type as input parameter for self.
*/
enum Message {
    Quit, // Variant has no data associated with it
    Move {x: i32, y: i32}, // Variant has named fields like a Struct
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Method body can be defined here.
        // The body of the method can use 'self' to access the variant instance of
        // the Enum which called this method.
    }
}

#[derive(Debug)] // For adding the 'Debug' trait into this custom Enum type, so that
// it can be printed using '{:?}' format placeholder.
enum UsState {
    Alabama,
    Alaska,
    Virginia,
    Louisiana,
    Michigan,
    Washington,
    Texas
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Virginia => year >= 1921,
            UsState::Louisiana => year >= 1898,
            UsState::Michigan => year >= 1901,
            UsState::Washington => year >= 1867,
            UsState::Texas => year >= 1987,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // We can create instances of each of the variants of an enum.
    // The variants of an enum are namespaced under its identifier and are accessed using
    // double colons "::". Also note that the variants of the enum are of the same type.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Since the variants of an enum are the same type, we can use the enum type as parameter
    // for a function and call the function on all variants of the enum
    route(four);
    route(six);

    /*
    Attaching data to each variant of the Enum. Note that the name of each Enum variant
    also becomes a function that constructs an instance of the Enum. IpAddrKind2::V4()
    is a function call that takes String as an argument and returns an instance of
    IpAddrKind2 type. We automatically get this constructor as a result of defining
    the Enum.
    */
    let _home = IpAddrKind2::V4(127, 0, 0, 1);
    let _loop_back = IpAddrKind2::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello")); // Creating a variant instance of 'Message' Enum
    m.call(); // calling an Enum method on its variant instance.

    /*
    One of the common in-built Enums used in Rust is the Option enum. It is defined by the
    standard library of Rust as follows:

    enum Option<T> {
        None,
        Some(T),
    }

    The Option Enum is already included into the prelude and does not need to be included into the scope.
    Rust does not have a null value and this Option Enum is used instead, to encode the concept of a
    value being present or absent. The <T> in the syntax signifies a generic type parameter. It means
    that the 'Some' variant of 'Option' Enum can hold one piece of data of any type, and each concrete
    type used in place of 'T' make the overall 'Option<T>' a different type.
    */

    let some_number = Some(5); // Using the Enum variant directly from the prelude. The type is Option<i32>
    let some_char = Some('e'); // This type is Option<char>
    let absent_number: Option<i32> = None; // Here, Rust cannot infer the type that the corresponding 'Some'
    // variant will hold by looking only at 'None' value.  So, we tell Rust that we mean for absent_number
    // to be of type Option<i32>.

    /*
    When we have a 'Some' value, we know that a value is present, and the value is present within 'Some'.
    When we have a 'None' value, in some sense, it means the same thing as null: no valid value.
    Because Option<T> and T (where T can be any type) are different types, the Rust compiler won't let
    us use an Option<T> value as if it were definitely a valid value. When we have a value of type i8,
    the Rust compiler will ensure that it is a valid value. Only when we have an Option<i8> type, we
    have to consider the case where it is possible to have no value. In other words, we have to convert
    value of Option<T> type to T type before we can do operations of T type on the value. So, everywhere
    that the value can possibly be null, use Option<T> type and everywhere you are sure that the value
    can never be null, use T type. You can handle each variant of Option<T> using a match expression.
    */
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // This will Error out!

    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // The "other" catch-all pattern always comes in the last arm to handle unknown or
        // invalid cases. It is used to store the value which did not match with all the arms
        // of the match expression and handle it appropriately.
        other => move_player(other), // Note that this compiles even though the move_player
        // function requires a u8 type parameter and at this point in the code, we do not know
        // the type of the "other" variable.
    };

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // we can also use the "_" catch-all pattern when we don't want to use the invalid value
        // that did not match the valid arms of the match expression.
        _ => reroll(), // We can also have an unit value '()' in this arm's execution code if we
        // want to do nothing, instead of the reroll() call.
    };

    /*
    While the match expression allows to handle all variants of an Enum, we can use the
    if .. let .. else control flow syntax to check for a single variant of the Enum and
    perform some logic if the variant matches. The if .. let .. else syntax also allows
    binding values to variables in the pattern matched.
    */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let Some(msg) = describe_state_quarter(Coin::Quarter(UsState::Texas)) else {
        return; // End main function if match does not occur
    };
    println!("{msg}");
}

fn route(_ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    /*
    Using a match expression with patterns for selective code execution:
    The match expression is a control flow construct that, when used with Enums, allows to selectively
    executes different code depending on which variant of Enum it has, and that code can use the data
    inside the matching value. It compares a value against a series of patterns and then execute code
    depending on which pattern matches. These patterns can be made of literal values, variable names,
    wildcards and many other things. Each arm is seperated from the next by a comma. When the match
    expression executes, it compares the resultant value against the pattern of each arm in order.
    NOTE: Match expressions on Enum have an arm to handle every variant of the Enum, else it
    will lead to compilation error! Rust will know that we did not cover every possible case in
    the match expression as matches in Rust are "exhaustive".
    We can use the "other" pattern as a last arm pattern to handle cases which do not match
    any variants of the Enum (it is called the catch-all pattern to handle edge cases).
    */
    match coin {
        /*
        These are the arms of the match expression. Each arm contains a pattern and code to run
        seperated by the "=>" operator. The code associated with each arm is an expression, and
        the resultant value in the matching arm of the expression is the value that gets returned
        for the entire match expression. We don't typically use curly braces if the match arm
        code is short.
        */
        Coin::Penny => {
            println!("Lucky Penny!");
            1 // This value is returned from this arm
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { /*
        Note that we match the data in the pattern to a variable 'state'
        This binds (extracts) the values from the matching input into the variables in the
        match pattern of the matched variant. The code in the matched variant's arm can then
        use this variable.
        */
            println!("State quarter from {state:?}!"); // Here, we inject variable into the format
            // string instead of supplying it as a parameter to the macro to put into the
            // placeholder in the format string.
            25 // This value is returned from this arm
        } // Lack of semicolon here means the match expression is evaluated and value is returned

    } // Note the lack of semi-colon here, which means, it returns the value from evaluation the matching pattern's code which in this case
    // is a number literal that will evaluate to its value and just be returned as it is.
}

// Function to increment the value of an Option<T> Enum type if a value exists in it
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // handle both variants of the Option Enum type. Note that since the Option type
        // is already included in the prelude, we do not import it here.
        None => None,
        Some(i) => Some(i + 1),
    } // Lack of semicolon here means the match expression is evaluated and value is returned
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn move_player(num_spaces: u8) {}

fn describe_state_quarter (coin: Coin) -> Option<String> {
    /*
    Note that the outer 'state' variable is different from the 'state' variable used
    in the pattern for binding value! The "if let .. else" syntax based expression
    returns the value of its evaluation (binding value from matching input to an inner
    scoped 'state' variable and returning that variable, or returning None if no match with
    input) to the outer scope 'state' variable.
    let state = if let Coin::Quarter(state) = coin { // match the input variant of Coin type Enum with
        // the Coin::Quarter pattern and if it is matched, execute the code below.
        state // return the bound value from the matched expression
    } else { // If no match occurs with expression
        return None; // Return the None variant of Option enum type and exit the function
    };

    We can further simplify this by using "let .. else" syntax without the "if" statement. Here,
    the "let" statement tries to match the pattern on the left with the expression on the right
    and if a match occurs, variables in the pattern are bound to corresponding values from the
    matching expression. This syntax is useful in cases where you have binding variable(s) in
    your variant matching pattern. If a match occurs, the extracted variable is available outsidde
    the scope of the let {} .. else {} expression - in this case, it is the describe_state_quarter
    function. The else block in this syntax MUST return or diverge out of the current execution path
    ie, break/continue out of the loop, or return from function that contains this syntax. This is
    different from the "if let {} .. else {}" syntax where the extracted variable from the matched
    expression is only available within the "if" block and no diverging in required in the else
    block.
    */
    let Coin::Quarter(state) = coin else { // assaigns value to 'state' variable if match occurs
        return None; // Returns None variant of Option and exits this function
        // This return statement is a MUST to prevent further execution of this function
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}