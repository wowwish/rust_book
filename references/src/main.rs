
/*
A reference is like a pointer in that it's an address we can follow to access the data
stored at that address; that data is owned by some other variable. Unlike a pointer, a 
reference is guaranteed to point to a valid value of a particular type for the life of
that reference.
*/
fn main() {
    let s1 = String::from("hello");

    // "&s1" is the syntax to create a reference that refers to the value of s1.
    // The action of creating a reference is called as "borrowing" in Rust.
    // NOTE: The opposite of referencing is dereferencing where the value can be accessed
    // from the reference using "*" as dereferencing operator.
    let len = calculate_length(&s1); // Here we send the reference to s1 instead of moving it
    
    println!("The length of '{s1}' is {len}");

    // change(&s1); // This will throw error as we are not allowed to modify references of immutable
    // variables.

    // Rust forces us to be very clear when we want to mutate a value using a function:
    // We have to add the "mut" keyword in variable declaration, function definition
    // and reference creation.
    let mut s2 = String::from("ahoy"); // The variable also needs to be declared as mutable
    change_mut(&mut s2); // Move a mutable reference to the function call

    // Mutable references have one caveat: if you have mutable reference to a value on heap,
    // you can have no other references to that value.
    // let r1 = &mut s2; // creating a fresh new mutable reference
    // let r2 = &mut s2; // creating a second new mutable reference for the same value - Error
    // will be thrown when it is used.
    // println!("{r1}, {r2}"); // This will throw error.
    /* This scenario depicts a "data race" similar to a race condition, that happens when:
            - two or more pointers access the same data at the same time
            - At least one of the two pointers is used to write to the data
            - There's no mechanism used to synchronize access to the data between the pointers
        Rust refuses to compile code with data races.
    */

    // Rust enforces a similar rule for combining mutable and immutable references.
    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM - we also cannot have a mutable reference when we already
    // have an immutable reference to the same value.
    // println!("{r1}, {r2} and {r3}");
    // This is because users of an immutable reference don't expect the value to change out
    // from under them! However, multiple immutable references are allowed because they cannot
    // affect anyone else's reading of data.

    // This will work because the scope of r1 and r2 ends after the println macro usage. Their
    // ownership is moved into the macro call.
    println!("{r1}, {r2}");
    let r3 = &mut s2; // no problem
    println!("{r3}");

    // Creating a dangling reference will throw compilation error in Rust.
    // let _reference_to_nothing = dangle();

    // The line below forces the Rust compiler to ignore compile-time warnings about variables
    // declared as mutable, but never modified within the scope of the variable.
    // This is useful when we want to demonstrate the concept of mutable references without actually
    // modifying the variable. This directive is called as "allow" attribute in Rust. It applies
    // only to the next line of code directly after it. It is a form of "attribute" in Rust that
    // can be applied to various items like functions, structs, enums, modules, etc. to modify
    // their behavior or provide additional information to the compiler.
    // Attributes are specified using the "#" symbol followed by square brackets containing the
    // attribute name and any optional parameters.
    #[allow(unused_mut)]
    let mut st = String::from("hello world");
    // let _word = first_word(&st); // word will get the value 5. This function call does not
    // affect the clear() method call below, but it is important to note that the value of word is
    // based on the original value of st. If we modify st, word will become invalid to use
    // because it is an index based on the original value of st.
    let word = first_word_slice(&st); // word will get the value "hello". This function call returns
    // a string slice and hence will not allow the invalid mutation of the underlying string st using
    // clear() method call on st, because the slice type now directly references the underlying string
    // instead of using index-based access.

    // st.clear();
    /* clear() will throw an error when first_word_slice is used.
    This call empties the String, making it equal to "", only when 'first_word' call is used.
    'word' still has the value 5 here, but st no longer has any context that we
    could meaningfully use with the value 5, so word is now totally invalid to use!
    When st is modified, word becomes invalid because it is an index based on the original
    value of st. However, compilation will not throw an error here because Rust does not know
    that the index stored in word is being used to access values of st later.
    
    A better way to handle this is to use a string slice instead of an index. When a string slice
    is used, Rust will throw a compile-time error if the underlying string is modified after
    the slice is created. This is because the slice type is an immutable reference to the
    underlying string and .clear() method call uses a mutable reference to truncate the
    underlying string value. Recall from the previous discussions on borrowing rules, that Rust 
    does not allow mutable references to be created when there are immutable references to the
    same value. The clear() method is has the function definition as follows:
    // pub fn clear(&mut self) {...}

    A string slice is an immutable reference to a contiguous sequence of elements of a String.
    A range is used to specify the start (inclusive) and end (exclusive) index of the slice.
    The slice type is stored in the Stack memory as a pointer to the byte at the start index
    and the length of the slice. This makes it efficient to access the slice compared to
    using an index to access the original String. The slice type is denoted by "&str" in Rust.
    Note that references to normal String type are denoted by "&String" in contrast.
    The slice type is a reference type, so it does not have ownership of the data it points to.
    String Literals are stored in the compiled binary of program as slice type (&str) and
    hence are immutable.
    */

    println!("The first word is: {word}"); // The immutable reference of the slice type is used here.

    let _hello = &st[0..6]; // In Rust's range syntax, if you want to start from index 0, you can
    // leave it out. So, the above line can also be written as:
    // let helo = &st[..6];
    let _world = &st[6..11];
    /*
    You can also leave out the end index to indicate that you want a slice that goes all the way
    to the end of the String. So, the above line can also be written as:
    let world = &st[6..];
    To get the entire string as a slice, you can leave out both the start and end index:
    let hello_world = &st[..];
    The above line is equivalent to:
    let hello_world = &st[0..st.len()];
    Also note that string slice range indexes must occur at valid UTF-8 character boundaries.
    For example, the following code will throw an error:
    let s = String::from("Здравствуйте");
    let slice = &s[0..1]; // This will throw an error because the first character in the string is
    a 2-byte character, and the range 0..1 does not include the entire character. The range
    must be 0..2 to include the entire character.
    */
    
    let my_string = String::from("hello world"); // Immutable String type variable
    // `first_word_slice` works on slices of `String`s, whether partial or whole.
    let _word = first_word_slice(&my_string[0..6]);
    let _word = first_word_slice(&my_string[..]);
    // `first_word_slice` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let _word = first_word_slice(&my_string);
    
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole.
    let _word = first_word_slice(&my_string_literal[0..6]);
    let _word = first_word_slice(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_slice(my_string_literal);
    

    // Furthermore, the slice type can be used to create slices of arrays and other collections
    // that are stored on the Heap as well.
    let a = [1, 2, 3, 4, 5]; // An array of integers
    let slice = &a[1..3]; // A slice of the array a from index 1 to 3 (exclusive).
    assert_eq!(slice, &[2, 3]); // The slice is equal to the array [2, 3]. The slice type is
    // denoted by "&[T]" where T is the type of the elements in the array. In this case: &[i32]
    // The slice type is a reference type, so it does not have ownership of the data it points to.
    // assert_eq! is a macro that checks if the two arguments are equal and panics if they are not.
}

fn calculate_length(s: &String) -> usize { // Notice that we take s, a reference to a String type
    // as the argument here. This allows the function to access the data without taking
    // ownership of it.
    s.len() // no semicolon, returns the length of String s
} // Here, s goes out of scope. But because s does not have ownership of what it refers to,
// the String s1 is not dropped.

// This function will throw compile-time error as we modify an immutable reference's value
// inside it.
// fn change(some_string: &String) { // the type has to be "&mut String" if we want to modify
//     // the underlying value of the reference.
//     some_string.push_str(" , world!");
// }

fn change_mut(some_str: &mut String) { // Here, the reference is moved into this function
    some_str.push_str(" , world!");
} // Here, the reference goes out of scope and is dropped.

// Function to create a dangling reference - a pointer that references a freed up 
// memory location (that may have been given to some other reference or variable).
// This function will not compile because it returns a reference to a String that
// is created inside the function!
// fn dangle() -> &String { // returns a reference to a String.
//     let s = String::from("hello"); // s is a new String.
//     &s // Returns the reference to a String s.
//     // This reference when used outside this function call is no longer pointing to data as
//     // the underlying data has been freed up from memory. Such a reference is called as
//     // "dangling reference".
// } // s goes out of scope and is dropped, so its memory goes away.
// // &s becomes a dangling reference. Danger!

/*
Slices allow you to reference contiguous sequence of elements in a collection on the Heap.
Collections in Rust can contain multiple values, and they are unlike
the built-in data types of Rust like Arrays and Tuples, in that they point to data stored in
Heap, which means the amount of data can grow or shrink and does not need to be known at
compile-time. Examples include: vector, string and hash map.
A Slice is a kind of reference and hence, it does not have ownership.
String Literals are stored in the compiled binary of program as slice type (&str) and
hence are immutable.
*/

// A function that takes a string of words seperated by spaces and returns the first word it
// finds, or the entire string if no spaces are found in the string.
fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Because we need to go through the string element by element,
    // we convert the string into an array of bytes. This is because strings are UTF-8 encoded
    // in Rust.

    for (i, &item) in bytes.iter().enumerate() { // .iter() returns an iterator over the array
        // of bytes. enumerate() returns a tuple of index and reference to the value of the byte
        // iterator at that index. We us pattern matching to destructure the tuple into the index
        // and the reference to the value at the index using (i, &item) pattern. The "&" is used
        // to dereference the reference to the value of byte at the index.
        if item == b' '{
            return i; // If the single byte is a space character, return the index at which
            // space was found. This becomes the length of the first word in the string.
            // The "b" before the single quote is used to denote that the character is a byte
            // literal.
        }
    }
    s.len() // Return the entire length of the string if no spaces were found in the string.
    // This is because the entire string is a single word in this case.
}

// In this modified function, we take in a slice type and return a slice type. The slice type
// can also be coreced from a String type, or a string literal, or a reference to a String type.
// This coercion is called as "Deref coercion" in Rust. It is a convenience feature of Rust that
// allows us to pass a reference to a String type to a function that expects a slice type.
fn first_word_slice(s: &str) -> &str { // This function returns a string slice
    let bytes = s.as_bytes(); // Because we need to go through the string element by element,
    // we convert the string into an array of bytes. This is because strings are UTF-8 encoded
    // in Rust.
    for (i, &item) in bytes.iter().enumerate() { // .iter() returns an iterator over the array
        // of bytes. enumerate() returns a tuple of index and reference to the value of the byte
        // iterator at that index. We us pattern matching to destructure the tuple into the index
        // and the reference to the value at the index using (i, &item) pattern. The "&" is used
        // to dereference the reference to the value of byte at the index.
        if item == b' '{
            return &s[0..i]; // If the single byte is a space character, return a slice of the
            // string from index 0 to i (exclusive). This becomes the first word in the string.
            // The "b" before the single quote is used to denote that the character is a byte
            // literal.
        }
    }
    &s[..] // Return a slice of the entire string if no spaces were found in the string.
    // This is because the entire string is a single word in this case. We need to index the
    // string with the full range of the string to return a slice of the entire string.
}