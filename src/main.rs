// This declaration will look for a file named `modules.rs` or `modules/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod modules;

// Three `use` declaration
use modules::function;
use modules::super_self::my_mod;
use modules::{call_public_function_in_modules as module_pub_fn, indirect_access};

fn main() {
    println!("Hello, world!\n");

    mod_output();
    struct_field();
    my_mod::indirect_call();
}

fn mod_output() {
    function();
    indirect_access();
    modules::public_function_in_crate();
    module_pub_fn();

    println!("\nEntering block");
    {
        // This is equivalent to `use modules::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::modules::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block\n");
    }
}

fn struct_field() {
    // Public structs with public fields can be constructed as usual
    let open_box = modules::my_struct::OpenBox { contents: "public" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    // let closed_box = modules::my_struct::ClosedBox {
    //     contents: "classified information",
    // };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = modules::my_struct::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
