mod modules;

fn main() {
    println!("Hello, world!");
    modules::function();
    modules::indirect_access();
    modules::nested::function();
    modules::public_function_in_crate();
    modules::call_public_function_in_my_mod();

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
