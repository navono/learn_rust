// By default, the items in a module have private visibility, but this can be overridden with the `pub` modifier.
// Only the public items of a module can be accessed from outside the module scope

// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;

// Modules can also be nested
pub mod nested;

pub mod super_self;

// Items in modules default to private visibility.
fn private_function() {
    println!("called `modules::private_function()`");
}

// Use the `pub` modifier to override default visibility.
pub fn function() {
    println!("called `modules::function()`");
}

// Items can access other items in the same module,
// even when private.
pub fn indirect_access() {
    print!("\ncalled `modules::indirect_access()`, that\n> ");
    private_function();
}

pub fn call_public_function_in_modules() {
    print!("\ncalled `modules::call_public_function_in_modules()`, that\n> ");
    nested::public_function_in_modules();
    print!("> ");
    nested::public_function_in_super_mod();
}

// pub(crate) makes functions visible only within the current crate
pub(crate) fn public_function_in_crate() {
    println!("called `modules::public_function_in_crate()`");
}

pub mod my_struct {
    // A public struct with public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

// Nested modules follow the same rules for visibility
mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `modules::private_nested::function()`");
    }

    // Private parent items will still restrict the visibility of a child item,
    // even if it is declared as visible within a bigger scope.
    #[allow(dead_code)]
    pub(crate) fn restricted_function() {
        println!("called `modules::private_nested::restricted_function()`");
    }
}
