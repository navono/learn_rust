fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

pub mod my_mod {
    fn function() {
        println!("called `my_mod::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my_mod::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("\ncalled `my_mod::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my_mod`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();

        // We can also use `self` to access another module inside `my_mod`:
        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my_mod` module).
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::modules::super_self::cool::function as root_function;
            root_function();
        }
    }
}
