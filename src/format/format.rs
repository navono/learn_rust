use std::fmt;
use std::fmt::Formatter;

// Printing is handled by a series of macros defined in `std::fmt`
// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint!but a newline is appended.

// fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
// fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

// Implementing the fmt::Display trait automatically implements the ToString trait which allows us
// to convert the type to String.

pub fn print_format() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number = 1, width = 6);
    println!("{number:0<width$}", number = 1, width = 6);
    println!("{number:-<width$}", number = 1, width = 6);
    println!("{number:0^width$}", number = 1, width = 7);

    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "Lee");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    print_precision();

    print_custom_format();
}

fn print_precision() {
    /**
      For integral types, this is ignored.
      For floating-point types, this indicates how many digits after the decimal point should be printed.
      There are three possible ways to specify the desired precision:
      1. An integer .N:
        the integer N itself is the precision.
      2. An integer or name followed by dollar sign .N$:
         use format argument N (which must be a usize) as the precision.
      3. An asterisk .*:
        .* means that this {...} is associated with two format inputs rather than one:
        the first input holds the usize precision, and the second holds the value to print.
        Note that in this case, if one uses the format string {<arg>:<spec>.*}, then the <arg> part
        refers to the value to print, and the precision must come in the input preceding <arg>.
    */

    println!("\n");

    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    println!("Hello {0} is {1:.5}", "x", 0.01);

    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {second of next two args (0.01) with precision
    //                          specified in first of next two args (5)}
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg 2 (0.01) with precision
    //                          specified in its predecessor (5)}
    println!("Hello {} is {2:.*}", "x", 5, 0.01);

    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
    //                          in arg "prec" (5)}
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    println!("\n");
    println!(
        "{}, `{name:.*}` has 3 fractional digits",
        "Hello",
        3,
        name = 1234.56
    );
    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned characters",
        "Hello",
        3,
        name = "1234.56"
    );
}

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Vector2D {
    x: isize,
    y: isize,
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Binary for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();

        // Respect the formatting flags by using the helper method
        // `pad_integral` on the Formatter object. See the method
        // documentation for details, and the function `pad` can be used
        // to pad strings.
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{:.*}", decimals, magnitude);
        f.pad_integral(true, "", &string)
    }
}

fn print_custom_format() {
    println!("\n");
    let my_vector = Vector2D { x: 3, y: 4 };

    println!("{}", my_vector); // => "(3, 4)"
    println!("{:?}", my_vector); // => "Vector2D {x: 3, y:4}"
    println!("{:#?}", my_vector); // Pretty print
    println!("{:10.3b}", my_vector); // => "     5.000"
}
