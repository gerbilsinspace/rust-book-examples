fn main() {
    // If statements look like javascripts, but without brackets.
    // rust will not convert the statement into a boolean value
    // that is on us.

    let number = 5;

    if number > 0 {
        println!("number is greater than 0");
    } else if number < 0 {
        println!("number is less than 0");
    } else {
        println!("number is equal to 0");
    }

    // because if is an expression, we can use it to set a value of a variable.
    // Make sure that each possible outcome is the same type

    let lesser_than_zero = if number < 0 { "yes" } else { "no" };
    println!("{}", lesser_than_zero);
    let final_count = loop_with_loop(3);
    println!("Repeated {} times", final_count);

    let final_while_count = loop_with_while(3);
    println!("Repeated {} times using while", final_while_count);

    let num_array = [12, 23, 34, 45, 56];
    let final_for_count = loop_with_for(num_array);
    println!("{}", final_for_count);

    count_between_numbers_backwards(1, 5)
}

fn loop_with_loop (max_count: i32) -> i32 {
    let mut count = 0;

    loop {
        if count >= max_count {
            break count
        }

        count = count + 1;

        println!("Again");        
    }
}

fn loop_with_while (max_count: i32) -> i32 {
    let mut count = 0;

    while count < max_count {
       println!("Again"); 
       count += 1;       
    }

    count
}

fn loop_with_for (input: [i32; 5]) -> i32 {
    let mut count = 0;
    for i in input.iter() {
        count += 1;
        println!("{}", i);
    }

    count
}

fn count_between_numbers_backwards(num1: i32, num2: i32) {
    for number in (num1..num2).rev() {
        println!("{}", number);
    }
}