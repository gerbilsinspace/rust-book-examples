fn main() {
    {
        enum IPAddressKind {
            V4(String),
            V6(String)
        }


        let four = IPAddressKind::V4(String::from("127.0.0.1"));

        // Using a reference as we are doing the match with the same variable.
        // remember when working with a type that is of a variable length,
        // without a reference, we invalidate variables as we use them.
        if let IPAddressKind::V4(b) = &four {
            println!("IP Address v4 = {}", b);
        }

        match &four {
            IPAddressKind::V4(a) => {
                println!("{}", a);
            },
            IPAddressKind::V6(a) => {
                println!("{}", a);
            }
        }



        let six = IPAddressKind::V6(String::from("::1"));
        if let IPAddressKind::V6(a) = six {
            println!("IP Address v6 = {}", a)
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColour(i32, i32)
    }

    impl Message {
        fn call(&self) {
            println!("{}", "Hi");
        }
    }

    let finish = Message::Quit;

    if let Message::Quit = finish {
        println!("Quitting Time")
    }

    let move_message = Message::Move{ x: 1, y: 10};
    if let Message::Move{ x, y } = move_message {
        println!("Coordinates are: x = {}, y = {}", x, y)
    }

    let write = Message::Write(String::from("Hi"));
    write.call();
    match write {
        Message::Write(m) => println!("Message written: {}", m),
        _ => {}
    }

    let change_colour = Message::ChangeColour(12, 23);
    match change_colour {
        Message::ChangeColour(r, g) => println!("Colour is {} {}", r, g),
        _ => println!("")
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            _ => Some(1)
        }
    }

    println!("{:?}", plus_one(Some(12)));
    println!("{:?}", plus_one(None));
}
