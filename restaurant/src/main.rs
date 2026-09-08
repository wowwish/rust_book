fn main() {
    // Typically in projects containing both a library crate (src/lib.rs) and
    // a binary crate (src/main.rs), the functional implementations are defined
    // in the library crate and exposed publicly to the binary crate which
    // provides a command-line wrapper for the APIs of the library crate. This
    // lets other projects also benifit from the functionality that the package
    // provides because the library crate's code can be shared. The module tree
    // in such cases should be defined in src/lib.rs and any public items can
    // be used in the binary crate by starting paths with the name of the package.
    println!("Hello, world!");
}
