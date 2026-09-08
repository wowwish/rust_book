/*
A 'crate' is the smallest amount of code that the Rust compiler considers at a time.
Even a single rust code file compiled with rustc is considered as a crate.
Crates may contain "modules" (declared using 'mod' keyword) defined in other files that get
compiled with the crate.
A crate in Rust has two forms:
    * Binary Crate: These are programs that compile into an executable that can be run from
    the command-line. Such crates MUST have a 'main' function defined in them.
    * Library Crate: These crates donot compile into an executable and do not need to have
    the 'main' function in them. They usually define functionality to be shared with multiple
    other projects. The is the type of crate that Rustaceans refer to by the common word
    "crate".

The 'crate root' is a source file that the Rust compiler starts from when compiling and it makes
up the root module of the crate.

A 'package' in Rust is a bundle of one or more crates that provides a set of functionality. A
package contains a 'Cargo.toml' file that describes how to build the crates of the package. Cargo
itself is a package with a binary crate for its command-line use and a library crate whose APIs
are exposed in the binary crate. A package may contain as many binary crates as you like, but it
must contain at most only one library crate. Also, a package must contain atleast one crate,
which can be either binary or library crate.

The `cargo new <my-project>' command creates a new package. Here, cargo follows the convention that
'src/main.rs' is always the crate root of a binary crate with the same name of the package.
Similarly, cargo knows that if a package contains a 'src/lib.rs' file, it is a library crate with
the same name as the package name and assumes 'src/lib.rs' as the crate root. Cargo passes the
crate root to the Rust compiler to build the binary or library crate.

Here, this package contains only 'src/main.rs'. So, it is has only a Binary crate. If a package has
both 'src/main.rs' and 'src/lib.rs', then it contains two crates - a Binary crate and a Library
crate. A package can have multiple Binary crates by placing files in the 'src/bin' directory, each
file will be a seperate Binary crate.

When compiling, the Rust compiler first looks for code to compile in the crate root ('src/main.rs'
for Binary crates and 'src/lib.rs' for Library crates). In the crate root file, you can declare
new modules using the 'mod' keyword. Furthermore, you can define sub-modules in any file other
than the crate root.

Modules allow us to organize code within a crate for readability and easy reuse. Modules also allow
us to control the privacy of items because code within a module is private by default. Private
module items are internal implementation details not available for outside use.
Modules can hold definitions for Structs, Enums, constants, traits, functions etc.
*/


pub mod garden; // Public module declaration
// mod garden {...}
/*
The compiler will look for this module's code in these places:
    * Inline, within curly brackets that would replace the semi-colon in the above declaration
    * In file 'src/garden.rs'
    * In file 'src/garden/mod.rs'

Once a module is part of your crate, you can refer to code in that module from anywhere else in
that same crate, as long as the privacy rules allow. Code within a module is private from its
parent module by default. To make a module public, declare it with 'pub mod' keyword. To make
declared items within a public module as public as well, declare them with the 'pub' keyword.
The 'use' keyword is used to create shortcuts to module items to reduce repetition of long paths.
*/

use crate::garden::vegetables::Asparagus; /* Brings the Asparagus type declared in sub-module
into scope of this root crate. The type can now be simply referred to by its name 'Asparagus',
without the full path. The contents of crate roots ('src/main.rs' and 'src/lib.rs') form 
a module named 'crate' which forms the root of a module tree. Here, we provide the 'path' to
find the item similar to the path to a file. A 'path' in Rust can take two forms:
    * An absolute path: which is the full path to the item from the crate root.
    * A relative path: which starts from the current module, and uses 'self', 'super' or an
    identifier in the current module.
Both absolute and relative paths are followed by one or more identifiers that are seperated
by double colons '::'.
*/

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
