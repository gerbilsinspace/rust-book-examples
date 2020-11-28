fn main() {
    // by default, all values are immutable

    // In order to mutate a value, you need to add the mut keyword during definition.
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    // Where are defining a variable with let can is immutable by default, we can also create constants.
    // Constants are always immutable, and are set at build time.
    // You cannot store a value which is set at runtime in a const.
    // The type of a const must be annotated. Nothing about a const can be inferred, as it is set at compile time.
    const MAX_POINTS: u32 = 100_000;
    println!("The constant MAX_POINTS is {}", MAX_POINTS);

    // We can shadow an immutable variable, which lets us make transformations
    // on a variable, but then keep it immutable afterwards.

    let y = 10;
    println!("Y is {}", y);
    let y = y * 2;
    println!("Y, after shadowing, is now {}", y);

    // We can change the type of a shadowed variable duting the transform

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The amount of spaces was {}", spaces);

    // If we attempted to change the type of a mutated variable, we would get an error.
}
