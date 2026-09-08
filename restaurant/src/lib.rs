// Private module declaration for this Library crate
// Modules can hold definitions for Structs, Enums, constants, traits, functions etc.
mod front_of_house { // front_of_house is parent module of hosting and serving
    // In Rust, items in the parent module can't use private items inside child modules.
    // However, child modules can use private items in their parent module. This is because
    // child modules wrap and hide their implementation details
    // Rust does allow you to expose inner parts of child module's code to outer
    // ancestor modules by using the 'pub' keyword in front of the item you want to expose.
    // This is called 're-exporting'.

    // Nested private module (sibling module of serving, child module of front_of_house)
    // Using the 'pub' keyword in front of the module declaration makes the module public and
    // accessible from outside the parent module (front_of_house in this case). However, the
    // items inside the public module are still private by default and can't be accessed
    // from outside the module unless they are also declared public.
    pub mod hosting {
       // The 'pub' keyword in front of the function declaration makes the function public and
       // accessible from outside the module (hosting in this case).
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Another nested private module (sibling module of hosting, child module of front_of_house)
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
} // Note the lack of semi-colon means this module is exported from this crate.

/*
Module tree:

crate 
    └── front_of_house
    ├── hosting 
        ├── add_to_waitlist
        └── seat_at_table 
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
*/

// We can also declare public items outside modules that can still be accessed from
// the Library crate in other crates that utilize this crate.
// Here, eat_at_restautant() can access the module front_of_house even without it
// being public since it is defined in the same crate as front_of_house. However, items
// inside front_of_house can't be accessed from this function unless they are
// declared public (like hosting and add_to_waitlist in this case).
pub fn eat_at_restaurant() {
    // Absolute path (Generally preferred way to call functions from modules)
    // Here, crate is the crate root, which is the 'src/lib.rs' file of this crate.
    // The path starts from the crate root since add_to_waitlist is defined in the same
    // crate as this eat_at_restaurant function (the crate in this case is 'restaurant')
    // Note that front_of_house is not declared as public, but we can still access it
    // in this absolute path because this eat_at_restaurant function is defined at the
    // same level as front_of_house (the implicit 'crate' root module). Then, the call to
    // hosting and add_to_waitlist() works because both are defined as public.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path - starts from the name of module defined at the same level of the
    // module tree as this eat_at_restautant() function. Since front_of_house is also
    // defined at the same level as this function (the implicit 'crate' root module),
    // we can access it without it being declared as public, from within this function.
    // Then, the call to hosting and add_to_waitlist works because both are defined
    // as public.
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the Summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Changed our mind about what bread we'd like
    meal.toast = String::from("Wheat"); // Since the struct field toast is delcared as public,
    // we can access it from outside the back_of_house module and modify it.
    println!("I'd like {} toast please", meal.toast);

    // The code below will not compile because the field seasonal_fruit of the Breakfast struct
    // is private and can't be accessed from outside the back_of_house module.
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}

mod back_of_house {
    // The 'pub' keyword can be used to expose Structs, Enums as well
    // apart from modules and functions. However, the fields of the
    // Struct or Enum also have to be declared with the 'pub' keyword
    // to expose them. Even when a 'pub' keyword is used to declare a
    // Struct, all its fields will remain private unless prefixed with
    // the 'pub' keyword.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // Any implementation blocks of the publicly defined Structs
    // will be accessible through the Struct will only expose the
    // publicly defined items within the implementation block.
    impl Breakfast {
        // Use the 'pub' keyword to expose Struct associated items
        // from its implementation block. This function allows us to
        // build an instance of Breakfast struct even though the field
        // seasonal_fruit is private and can't be accessed from outside
        // the back_of_house module. This is a common pattern in Rust
        // to expose a public Struct with private fields and provide
        // public associated functions to build instances of the Struct.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast), // Use the function parameter to build String
                seasonal_fruit: String::from("peaches"),
            } // Note the lack of semicolon here. This is returned from this function.
        }
    }

    fn fix_incorrect_order() {
        cook_order(); // This function can access cook_order() without it being
        // a public declaration because cook_order() and fix_incorrect_order()
        // are defined at the same level (within this back_of_house module).
        // Using the 'super' keyword in Rust Paths allows us to reference
        // an item in the parent module of the current module. Here,
        // deliver_order() is declared in the parent module of back_of_house
        // (which in this case is 'crate') and we use the super keyword to
        // access it from there.
        super::deliver_order();
    }

    fn cook_order() {}

} // Note the lack of semi-colon means this module is exported from this crate.