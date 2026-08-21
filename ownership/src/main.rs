fn main() {
    /*
    Rust is a systems programming language and hence, it has to handle momory allocation
    carefully. The Stack and Heap are two parts of memory that programming languages have
    to manage during code runtime. The Stack stores data in Last-in-First-out (LIFO)
    fashion. The size of the data must be fixed throught runtime and must be a known
    value to put it in the Stack (eg. literals and scalar datatypes of rust like 
    i32, u32, bool, char). Adding data to a Stack is called "pushing onto the stack"
    and removing data from the Stack is called as "popping off the stack".
    The Heap is an unorganized memory storage where a certain amount of space is
    requested for storing data by the memory allocator, and data is put when a big
    enough empty spot is available, marking that spot with a "pointer" (which is the
    address of the location). This process is called as "allocating". Since the "pointer"
    is of known, fixed size (as it stores only memory address and not the actual data), it
    is stored in the stack after data "allocation" in the heap. (eg. With the String type,
    in order to support a mutable, growable piece of text, we need to allocate an amount of
    memory on the heap, unknown at compile time, to hold the contents).
    Pushing to the stack is faster than allocating on the heap because the allocator never
    has to search for a place to store new data; that location is always at the top of the stack.
    Comparatively, allocating space on the heap requires more work because the allocator must
    first find a big enough space to hold the data and then perform bookkeeping to prepare for
    the next allocation. Accessing data in the heap is generally slower than accessing data on
    the stack because you have to follow a pointer to get there. Contemporary processors are
    faster if they jump around less in memory. A processor can usually do its job better if
    it works on data that’s close to other data (as it is on the stack) rather than farther
    away (as it can be on the heap).
    When your code calls a function, the values passed into the function (including, potentially,
    pointers to data on the heap) and the function’s local variables get pushed onto the stack.
    When the function is over, those values get popped off the stack.
    
    Unlike other Programming languages, in Rust, memory allocated to a variable is
    deallocated as soon as the variable goes out of scope. Rust Ownership rules:
        - Each value in Rust has an "owner" (a variable).
        - There can only be one "owner" at a time.
        - When the "owner" goes out of scope, the value will be dropped.
    */
    let s1 = String::from("Hello"); /* data is allocated in the Heap and the pointer to the data,
    the string length and current holding capacity of the stored location is stored in the Stack.
    In this case, the heap will hold the literal "Hello" and the stack will hold:
    [ptr: pointer, len: 5, capacity: 5]
    */
    let s2 = s1; /* s1 is invalidated. s1's contents of the stack are "moved" from s1 to s2.
    The heap data remains. This is done because, in Rust, when both s1 and s2 here go out
    of scope (after main function exits), deallocation will be triggered once for s1 and once
    for s2. Without s1 invalidation, both s1 and s2 will point to the same data on the heap
    and the data cannot be removed twice from the heap (double free error). When variables
    like s1 or s2 go out of scope, Rust calls a special "drop" function on the variable where
    the variable type would have logic for freeing up and returning memory associated with
    that variable.
    */
    
    // println!("{s1}, world!"); // Will throw error

    let mut s = String::from("hello"); // s comes into scope
    s = String::from("ahoy"); // In this case, nothing is now referring to the previous data
    // "hello" that was allocated in the heap. Thus, the original string entity goes out of scope
    // and Rust will call the "drop" function on it to free its memory at this point.
    println!("{s}, world!");

    // In order to copy the entire Stack+Heap data of an existing variable into a new variable
    // (intead of performing a "move"), we can use the "clone" method.
    let s3 = s2.clone();

    let x = 5;
    let y = x;
    /*
    In this case, both x and y are of known, fixed size and are stored as the default i32
    type in the Stack during compilation. So, copies of their values are easier to make.
    Rust has a special annotation called the "Copy" trait that we can place on types that are
    stored on the stack, as integers are. If a type implements the "Copy" trait, variables that
    use it do not move, but rather are trivially copied, making them still valid after
    assignment to another variable. All integer and floating-point types, bool, char
    all have the "Copy" trait and behave this way. Tuples containing only types that implement
    the "Copy" trait also have this behaviour.
    */
    println!("x = {x}, y = {y}");

    // Passing a variable to a function will move or copy, just as assignment does.
    take_ownership(s); // S's value moves into the function ... and so, it is no longer valid.
    // println!("{s}, world!"); // This will throw compile-time error
    make_copy(x); // Because i32 implements the "Copy" trait, x does NOT move into the function.
    // so, it is okay to use x afterward.

    // NOTE: unused variables are prefixed with underscore in Rust to prevent compiler warnings.
    let _a1 = gives_ownership(); // gives_ownership moves its return value into a1
    let _s4 = takes_and_gives_back(s3); // s3 is moved into takes_and_gives_back, which
    // also moves its return value into s4.

    let s5, len = calculate_length(s2); // s2 moves into the function .. so it is no longer valid.
    println!("The length of '{s5}' is {len}")
    /*
    While this works, taking ownership and then returning ownership with every function is a bit
    tedious. What if we want to let a function use a value but not take ownership? It’s quite
    annoying that anything we pass in also needs to be passed back if we want to use it again,
    in addition to any data resulting from the body of the function that we might want to return
    as well. Luckily for us, Rust has a feature for using a value without transferring
    ownership, called "references".
    */
} /*
Here, a1, s4, s5 and len go out of scope and are dropped from Heap + Stack.
s1, s2, s3 and s were moved, so nothing happens.
x and y also become out of scope and get dropped from the Stack.
The ownership of a variable follows the same pattern every time: Assigning a value to
another variable moves it. When a variable that includes data on the heap goes out of scope,
the value will becleaned up by "drop" unless ownership of the data has been moved to another
variable.
*/

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn make_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned (no semicolon) and moves out to the calling function
}

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned (no semicolon) and moves out to the calling function.
}

// A function that returns multiple values. In this case, we return the original argument
// moved into the function as well, to transfer its ownership back to the caller.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // returning multiple values using a tuple
}