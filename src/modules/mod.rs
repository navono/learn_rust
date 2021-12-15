// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;

// Modules can also be nested
pub mod nested;

// Items in modules default to private visibility.
fn private_function() {
    println!("called `my_mod::private_function()`");
}

// Use the `pub` modifier to override default visibility.
pub fn function() {
    println!("called `my_mod::function()`");
}

// Items can access other items in the same module,
// even when private.
pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
}

pub fn call_public_function_in_my_mod() {
    print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
    nested::public_function_in_my_mod();
    print!("> ");
    nested::public_function_in_super_mod();
}

// pub(crate) makes functions visible only within the current crate
pub(crate) fn public_function_in_crate() {
    println!("called `my_mod::public_function_in_crate()`");
}

// Nested modules follow the same rules for visibility
mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `my_mod::private_nested::function()`");
    }

    // Private parent items will still restrict the visibility of a child item,
    // even if it is declared as visible within a bigger scope.
    #[allow(dead_code)]
    pub(crate) fn restricted_function() {
        println!("called `my_mod::private_nested::restricted_function()`");
    }
}
