fn main() {
    // We can have integers, which come in 8, 16, 32, 64, and size variants.
    let num8: i8 = -8;
    let num16: i16 = -16;
    let num32: i32 = -32;
    let num64: i64 = -64;
    let numarch: isize = -64;
    println!("{} {} {} {} {}", num8, num16, num32, num64, numarch);

    // We can have unsigned integers, which come in 8, 16, 32, 64, and size variants.
    let unum8: u8 = 8;
    let unum16: u16 = 16;
    let unum32: u32 = 32;
    let unum64: u64 = 64;
    let unumarch: usize = 64;
    println!("{} {} {} {} {}", unum8, unum16, unum32, unum64, unumarch);

    // For floats, we have two types, 32 and 64 bit floating point numbers.
    // 64 point float is the default when not set.

    let fnum32: f32 = 2.0;
    let fnum64: f64 = 3.0;

    println!("{} {}", fnum32, fnum64);

    // We can do the usual numeric operations: +, -, *, /, and %
    // % is for remainder, not percent.

    // We get boolean operators, which are 1 bit, true and false values

    let boolean: bool = true;
    println!("{}", boolean);

    // We have a specific character type
    let c = 'c';
    println!("{}", c);

    // We have tuples, which are a nice way of grouping together a set of values
    // with a variety of types into one compound type.

    let tup: (&str, i32) = ("Age", 30);
    println!("{}: {}", tup.0, tup.1);

    // And we have arrays. all values in an array must be the same type

    let numbers = [1, 2, 3, 4, 5];
    println!("{}", numbers[0]);

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    println!("There are {} months", months.len());

    // you can set the amount of items in the array, and a default value, at init
    let scores = [0; 5];
    println!("There are {} scores, which are {}", scores.len(), scores[0]);

    // Rust requires you to set the amount of items in array up front
    // so it can provide you with some nice compile time error checking.
    // You either fill the array at init, or you tell it at init how many items to expect.
    // This prevents the program from accessing memory not allocated to the array.

    print("Hello from another function");
    printnum(5);

    // Working with macros inside of a function allows you to return values from an expression.
    let y = using_macro(2);
    println!("{} {}", y[0], y[1]);
}

fn print(val: &str) {
    println!("{}", val);
}

fn printnum(val: i32) {
    println!("{}", val);
}

fn using_macro(val: i32) -> [i32; 2] {
    let x = 5;

    let y = {
        // Because we are inside a new scope, when we shadow the x variable
        // from outside, we will not overwrite it.
        let x = 0;

        // By leaving off the semicolon, we turn a statement
        // into an expression which will be returned from the function.
        x + val
    };

    [x, y]
}