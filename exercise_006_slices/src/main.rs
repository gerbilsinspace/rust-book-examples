fn main() {
    {
        // We can get a substring by using the range syntax
        let mut s = String::from("Hello World!");

        // References the same data as s does, but only refers to 0 to 4 chars.
        // Values before the second number are included.
        // The value of the second number is not included.
        let s1 = &s[0..5];

        // You can leave off the second number and it will include the rest of the string
        let s2 = &s[6..];

        // and leaving the starting number off will get you everything from the start of the string
        let s3 = &s[..5];

        // If you want a reference to everything (I don't know why you would) leave off both numbers
        let s4 = &s[..];

        // you cannot do minus numbers as that makes no sense at all
        // let s5 = &s[3..-1];

        println!("{} {} {}, {}", s1, s2, s3, s4);

        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }

            }

            &s[..]
        }

        // if you attempted to clear s before finding the first word below, you'd get an error
        // You would be trying to access data that has been cleared
        // s.clear();

        println!("{}", first_word(&s));

        s.clear();
    }

    {
        // you can also use the range syntax on arrays
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let b = &a[..4];
        println!("{}", b[1]);
    }
}
