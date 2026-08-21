
/*
Structs (Structures) are used to create custom user-defined data types in Rust. They are similar
to Objects with attributes from Object-Oriented-Programming in other programming languages.
Similar to tuples, structures can group tougher multiple related values that make up a 
meaningful named group. However, unlike tuples, the fields in a struct have names and you can
access them using dot notation without relying on the order of data to access them.
Structs are useful when you want to create a complex data type that has multiple attributes.
Structs can also be used to create Methods - functions that operate on the data within the struct.
*/

#[allow(dead_code)] // This attribute is used to suppress the warning for unused code. It is used
// here because some fields of the User struct are not used in this example. They are for
// demonstration purposes only. In a real-world application, all fields of a struct would be used in
// the code that declares the struct.
struct User { // A struct's name is always singular and capitalized by convention. It describes the
// significance of all the fields within the struct.
    // Fields
    username: String, /*
    using String here is a deliberate choice. Using slice type "&str" will require the concept
    of lifetimes in Rust because slices are references and we want struct instances
    to own their data, and the data must be valid for the entire lifetime of the struct. In this
    example, we simply use String to avoid the complexity of Rust lifetimes.
    */
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // We set this attribute to 
struct Rectangle {
    width: u32,
    height: u32,
}

/*
An implementation block associates all definitions within it, with a specific type.
In this case, all definitions within this block is associated with the Rectangle type.
It helps with organizing all stuff related to an instance of a  type in one place.
Note that you can have multiple implementation blocks defined for the same type and their
contents will be collated under the type.
*/
impl Rectangle {
    /*
    Method declaration for Struct instances. Methods are functions defined within the context
    of Structs, Enums, Traits etc, and their first parameter is always "&self", which represents
    the instance of the Struct that the method is being called on. The "&self" is actually
    a shorthand for "self: &Self". Within an implementation block, the "Self" is an alias for
    the type that the implementation block is for. Note that we borrow here using "&self".
    Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow
    self mutably, just as they can any other parameter. If we want to change the struct instance
    as part of the method, we would use "&mut self" as the first parameter of the method. Having
    a method that takes ownership of the instance by using "self" as first parameter is rare, and
    used only in cases where the method completely transforms the instance and you want the caller
    to stop using the original instance after the transformation (remember that the original
    instance will be dropped once ownership is transferred to the method and the method completes
    and execution goes back to the caller).
    When you call a method with instance.method(), Rust automatically adds in "&", "&mut" or "*"
    so that the instance matches the first parameter of the method signature. Hence, 
    p1.distance(&p2) and (&p1).distance(&p2) are the same. This automatic referencing behaviour
    works because given the receiver (first parameter) and name of a method, Rust can
    deterministically figure out whether the method is reading (&self), mutating (&mut self) or
    consuming (self).
    */
    fn area(&self) -> u32 {
        self.width * self.height // Note the lack of semicolon here which means this computed
        // value is returned. Also note how we access the struct instance fields using "self" and
        // dot notation within the method. This works although we have only a pointer reference
        // to Self instance because, Rust performs automatic referencing and dereferencing.
    }
    // Methods can have the same name as one of the fields of the Struct. Such methods are often
    // used as "getters" for Struct fields. Rust does not implement getters for Struct fields
    // automatically, but they can be useful when you have private struct fields that need
    // read-only access using a public method.
    fn width(&self) -> bool {
        self.width > 0
    }

    // Method with multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
    Keep in mind that all functions declated within an implementation block are called as
    "associated funtions" including methods themselves. We can have "associated functions" within
    an implementation block that do not act on an instance of the type of the implementation block,
    but provide additional functionality to the type. An example is a constructor which creates a
    new instance of the type (usually called as "new"). We can for example have a associated
    function for Rectangle called "square" that takes one dimension parameter and uses it
    to build a Rectangle with equal width and height. To call an associated function that is
    not a method of the type, we use the "::" operator with the struct name.
    */
    fn square(size: u32) -> Self {
        // The Self keyword in the return type and body of this function are aliases for the type
        // of this implementation block.
        Self {
            width: size,
            height: size
        } // Note the lack of semicolon here which means this instance is returned.
    }
}


// Rust also supports structs that have only field types and lack field names. These are
// called tuple structs.
#[allow(dead_code)] // This attribute is used to suppress the warning for unused code. It is used
// here because the fields of the Color tuple struct are not used in this example code.
struct Color(i32, i32, i32); // A tuple struct that represents a color in RGB format.
struct Point(i32, i32, i32); // A tuple struct that represents a point in 3D space.

// A unit-like struct
struct AlwaysEqual; // A unit-like struct that has no fields. It is useful when you want to
// implement a trait on some type but don't have any data to store in the type.

fn main() {
    // Creating an instance of a struct is called instantiation. You can instantiate a struct
    // by using the struct name followed by curly braces containing the values for each field. The
    // order of the fields does not matter as long as you specify the field names.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    /*
    Mutating a struct's field is done by using dot notation to access the field and then
    assigning a new value to it. The instance of the struct must be mutable in order to change
    its fields. Rust does not allow only certain fields of a struct to be mutable, so if you want
    to change a field, you must declare the entire struct instance as mutable.
    */
    user1.email = String::from("anotheremail@example.com");

    /*
    It is often useful to create a new instance of a struct based on an existing instance.
    let user2 = User { 
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count, 
    };
    You can also do this by using the struct update syntax, which allows you to specify the fields
    you want to change and then use the .. syntax to specify that the remaining fields should be 
    copied from an existing instance. This is useful when you want to create a new instance that is
    similar to an existing instance but with some fields changed.
    */
    let _user2 = User {
        // The fieds that you want to change must be specified first, followed by the .. syntax
        // to specify that the remaining fields should be copied from an existing instance.
        email: String::from("another@example.com"),
        ..user1 /*
        This copies the remaining fields from user1 to user2. Note that user1 must be
        mutable in order to use the struct update syntax. This is because the struct update syntax
        moves the fields from the existing instance to the new instance. If the existing instance
        is not mutable, then the fields cannot be moved and the code will not compile. This also
        means that user1 is not longer valid after this point because its fields have been moved
        to user2. user1 contails two String fields which are heap allocated and the ownership
        of the heap allocated memory is moved to user2. If instead, we provided new values for the
        String fields of user2 and used the struct update syntax to copy the remaining fields from
        user1, then user1 would still be valid after creating user2! This is because bool and u64
        are Stack-only types with the Copy trait and their values are copied to user2 instead of
        moved. If you want to keep user1 valid, you can use the clone method to create a copy
        of the fields instead of moving.
        */
    };

    // Note that these tuple struct instances are different types and cannot be used interchangeably
    // even though they have th same field types and same number of fields in the same order.
    // This is because they have different struct names and hence, are considered as different types
    // by the Rust compiler.
    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    Fields of tuple struct instances can be accessed using dot notation and the index of the
    field. The index starts at 0 for the first field, 1 for the second field, and so on. This
    is similar to accessing the elements of a tuple using dot notation and the index of the
    element. 
    */
    println!("The origin is at ({}, {}, {})", origin.0, origin.1, origin.2);

    let Point(x, y, z) = origin; /*
    Destructuring a tuple struct into its individual fields. This is
    similar to destructuring a tuple into its individual values. The fields are assigned to the
    variables in the order they are defined in the struct. The variables must be declared in
    the same order as the fields in the struct. The variables can be used to access the
    individual fields of the struct.
    */
    println!("The origin is at ({}, {}, {})", x, y, z);

    // We can also have structs wit not fields. Such structs are called unit-like structs. They
    // are useful when you want to implement a trait on some type but don't have any data to
    // store in the type.
    let _subject = AlwaysEqual; // Creating an instance of a unit-like struct. Note that there are no
    // parentheses after the struct name. This is because unit-like structs do not have any fields.

    // let _user3 = build_user("someone@example.com", "someusername123"); // This
    // call will throw an error here asking for lifetime specifiers if we used &str instead of String
    // when declaring the User struct.

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    /*
    println!("rect1 is {rect1}"); // This will throw a compilation error because by default,
    the curly braces in the format string of println! tells it to use formatting by trait
    std::fmt::Display, which is not implemented by our Struct. Primitive types of Rust implement
    this trait, but for custom Struct types, the way to format output in println! is unclear due
    to many possibilities: do you want commas or not ? should curly brackets also be printed ? etc.
    Hence, by default, structs in Rust donot implement the Display trait.
    */
    println!("rect1 is {rect1:?}"); /*
    Puting the ":?"" specifier in the curly brackets tells println!
    that we want to use an output format called "Debug", which is a trait used to print structs
    in a useful way for developers. In order for this to work, you also need to add the
    #[derive(debug)] directive above the struct declaration.
    You can also use the ":#?" specifier to pretty-print with indentation. Also, you can use
    the dbg!() macro to take ownership of the value, print it along with file name, line
    number and variable or expression, then return the ownership of the value. Note that
    calling dbg!() prints to the standard error whereas println!() prints to the standard output.
    Rust provides a number of other traits for use with the "derive" attribute that can add
    useful behaviour to custom types like structs.
    */
    dbg!(&rect1);

    let scale = 2;
    let rect3 = Rectangle{
        width: dbg!(scale * 30), // We can use dbg! here because it returns ownership of the
        // expression's value. The width field will have the same value as if we didn't have
        // the dbg! call here.
        height: 45,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    // Method call
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Calling an associated function of the Rectangle struct
    let _sq = Rectangle::square(3);
}

#[allow(dead_code)] // This attribute is used to suppress the warning for unused code. It is used here
// because the build_user function is not used in this example. It is included here for
// demonstration purposes only.
// Function to build a new user instance. It makes sense to name the function parameters with
// the same name as the struct field names. This allows us to use the field init shorthand syntax
// to initialize the struct fields with the same name as the function parameters.
fn build_user(email: String, username: String) -> User {
    User{
        email, // Field init shorthand syntax. This is equivalent to email: email
        username, // Field init shorthand syntax. This is equivalent to username: username
        active: true,
        sign_in_count: 1,
    } // Note the lack of semicolon after struct instantiation. This returns the instance.
}

// Function to calculate the area of a rectangle. It takes a reference to a Rectangle struct as
// an argument. This is because we don't want to take ownership of the Rectangle instance. We
// just want to borrow it so that we can access its fields without taking ownership of it.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height // Accessing the fields of the Rectangle struct using
    // dot notation does not move the values of the fields, because it is a borrowed reference
    // to the Rectangle struct instance.
}