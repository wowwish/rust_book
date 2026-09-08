pub mod vegetables; // Public sub-module declaration (parent module is garden)
// mod vegetables {...}
/*
The compiler will look for submodule's code within the directory names for the parent module
in these places:
    * Inline, directly following the parent module, wihtin curly brackets instead of the semi-colon
    above.
    * In the file 'src/garden/vegetables.rs' (in this case, sub-module code is imported from here)
    * In the file 'src/garden/vegetables/mod.rs'
*/