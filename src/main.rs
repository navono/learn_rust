mod modules;

fn main() {
    println!("Hello, world!");
    modules::function();
    modules::indirect_access();
    modules::nested::function();
    modules::public_function_in_crate();
    modules::call_public_function_in_my_mod();
}
