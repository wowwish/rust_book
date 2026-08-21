fn main() {
    // Integer types:
    // i8 - size: 8 bits, can hold -128 to 127 - (-2^(n - 1)) to (2^(n - 1)) - 1 
    // u8 - size: 8 bits, can hold 0 to 255 - 0 to 2^n - 1)
    // i16/u16 - size: 16 bits
    // i32/u32: size - 32 bits (Default)
    // i64/u64: size - 64 bits
    // i128/u128: size - 128 bits
    // isize/usize: size - 32 bits or 64 bits depending on architecture of computer running program

    // Floating-Point Types:
    // f32: size - 32 bits
    // f64: size - 64 bits (Default)

    // Boolean Type:
    // bool: size - 1 byte  (8 bits)

    // Character Type:
    // char: size - 4 bytes (32 bits)

    // A tuple is an immutable compound data type in Rust that can hold more than one element
    // of any type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring the tuple
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // Indexing a tuple
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // An empty tuple is a type of its own called a unit type and it signifies absence of data
    let _empty = ();

    // Unlike a tuple, every element of an Array data-type in rust must be of the same type
    // Arrays also have a fixed length in rust.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5]; // shorthand for 'let b = [3, 3, 3, 3, 3];'

    // Array indexing
    let _first = a[0];
    let _second = a[1];

    // Out of range index used to access array elements will cause Rust to panic
    // let _invalid = a[5];
}
