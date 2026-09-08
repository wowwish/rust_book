#[derive(Debug)] // Attribute added to add implementation of std::fmt::Debug trait for the custom
// struct type declared below this line. This allows printing the struct in the println! macro
// using a debug formatter '{:?}'
pub struct Asparagus {} // Public item declaration for sub-module. Note the absence of 
// semi-colon here which means that this is exported out of this sub-module.