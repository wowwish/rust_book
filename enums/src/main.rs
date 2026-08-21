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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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

    /*
    The match expression is a control flow construct that, when used with Enums, allows to selectively
    executes different code depending on which variant of Enum it has, and that code can use the data
    inside the matching value. It compares a value against a series of patterns and then execute code
    depending on which pattern matches. These patterns can be made of literal values, variable names,
    wildcards and many other things.
    */


}

fn route(_ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    // Using a match expression with patterns for selective code execution
    match coin {
        // These are the arms of the match expression. Each arm contains a pattern and code to run
        // seperated by the "=>" operator.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    } // Note the lack of semi-colon here, which means, it returns the value from evaluation the matching pattern's code which in this case
    // is a number literal that will evaluate to its value and just be returned as it is.
}