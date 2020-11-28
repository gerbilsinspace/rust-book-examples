fn main() {
    // Variable scope differs between simple types and compound types.
    // Simple types are integers, floats, bools are chars, which have a set memory usage.
    // If a tuple has only these values in it, they also have a set memory usage, and can
    // work as if they were a simple type.

    // Types which have a set amount of memory are trivial to reason about when it comes
    // to freeing up said memory. We create the variable on the stack, and when we come
    // to the end of the scope the variable was created in, the memory is popped off of
    // the stack. Super speedy.

    // Things get more complex with types whose size is not known at compile time.
    // Strings are a good example. We cannot put the string on the stack, as it's size can fluctuate,
    // and it has to be allocated memory from the heap.

    {
        // _a is a 32 bit integer, which means we know that when _a
        // is not needed any more, we free up 32 bits of memory.

        let a: i32 = 1; // _a is created
        println!("{}", a);
        // a can be used
    } // a is freed

    {
        // _b is a boolean, which takes up a single bit, whose
        // value is 0 for false, and 1 for true.

        let b: bool = false; // b is created
        println!("{}", b); // b is used
    } // b is freed

    {
        // Because c is a string literal is set at compile time, we know it's length, so it will
        // be placed on the stack. Using this is blazing fast, but it does mean that the string
        // can never be mutated.

        let c = "Hello"; // c is created
        println!("{}", c); // c is used
    } // c is freed

    {
        // To mutate a string, you need to place it on the heap. This allows us to have a variable with
        // unknown size. This is done by calling the method String::from
        let mut d = String::from("Hello");

        d.push_str(", world!");
        println!("{}", d);
    }

    {
        // if you reassign a simple variable, like a number, it will act as a copy.
        let e = 5; // assign 5 to a
        let f = e; // copy the value of a and  assign it to b
        println!("{} {}", e, f);

        // we are able to copy the following values in this way:
        // integers, boolean, floats, char, and tuples if they only have the previous types and nothing else.

        // If we tried this with a variable which has an unknown size, we would get a move, rather than a copy
        let g = String::from("hi");
        let h = g;

        // the pointer to the data in c has now been assigned to d. If we referenced c we would crash.
        println!("h = {}", h);

        // If both variables were assigned to the pointer, and the first variable went out of scope, any references
        // from the second variable would now lead to an error as the memory had been freed. We would also get a second
        // error when the second variable tried to free the data, as that data will have already been freed.

        // to clone the data into a new variable, you can use the clone method
        let i = h.clone();
        println!("{}, {}", h, i);
    }

    {
        // In order for the function to not taked ownership of the variable it has been passed in
        // we give the function a reference to the string
        fn print_string (s: & String) {
            println!("{}", s);
        }

        // In order to modify the variable coming in, you need to explicitly say you will mutate it
        fn append_string(s: &mut String) {
            s.push_str(", world");
        }

        let mut j = String::from("Hey");
        print_string(& j);
        append_string(&mut j);
        print_string(& j);
        append_string(&mut j);
        print_string(& j);

        // At any given time, we can either have one immutable reference to data, or any number of immutable references.

        // Cannot create a second reference after creating a mutable reference later.
        // If we did, we might end up in a data race condition, because:
        //      * We have 2 pointers to the same data, at least one of which is mutable
        //      * We have no mechanism in place to make sure that the data is being synchronised correctly

        // let _copy_j = &mut j;
        // let copy_again_j = &mut j;
        // println!("{}, {}", copy_j, copy_again_j);

        // we also cannot have a mutable and an immutable reference to the same data,
        // as we do not want the immutable data to change from underneath us.

        // We need to make sure that if we create a variable inside a function, and want it to exist
        // outside of a function, we pass the data, and not a reference. Otherwise, the scope of the
        // new_string is the function, and it will be freed as we exit the function. By passing the value,
        // the ownership is passed also
        fn create_string() -> String {
            let new_string = String::from("Hi");
            new_string
        }

        let k = create_string();
        println!("{}", k);
    }
}
