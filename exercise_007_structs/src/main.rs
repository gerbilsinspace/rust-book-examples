fn main() {
    // Ooh grouped data
    struct Address {
        line1: String,
        line2: String,
        line3: String,
        lat: f64,
        lng: f64
    }

    // create our first address
    let address = Address {
        line1: String::from("Somewhere"),
        line2: String::from(""),
        line3: String::from(""),
        lat: 1.2345,
        lng: 23.123432
    };

    println!("{} {} {} {} {}", address.line1, address.line2, address.line3, address.lat, address.lng);

    // structs in structs in structs
    struct Office {
        name: String,
        address: Address
    }

    // This isn't a copy, it's a move. the address variable is now void
    // let address1 = address;

    // you can get attributes from other structs to act as defaults.
    // For simple types the data is copied over, but for strings they are moved
    // and the previous struct pointing to the data is invalidated
    let address1 = Address { lng: 23.45, ..address };

    // if we see what address.line1 would be, we would error as it has been moved
    println!("{}", address.lat);

    let office = Office {
        name: String::from("The Drey"),
        address: Address {
            lat: 1.2,
            ..address1
        }
    };

    let address2 = Address {
        ..office.address
    };

    // everything inside a mutable struct is mutable
    let mut office2 = Office {
        name: String::from("The Drey"),
        address: address2
    };

    office2.name.push_str(" 2");
    office2.address.line1.push_str(", hi");
    println!("{} {}", office2.name, office2.address.line1);

    fn build_office (name: String, line1: String) -> Office {
        let add = Address {
            line1,
            line2: String::from(""),
            line3: String::from(""),
            lat: 0.0,
            lng: 0.0
        };

        Office {
            name,
            address: add
        }
    }

    let office3 = build_office(String::from("The Drey 3"), String::from("Poop"));
    println!("{} {}", office3.name, office3.address.line1);

    let mut office4 = Office {
        ..office2
    };
    office4.name.push_str(", wait actually 4");
    println!("{}", office4.name);

    // You can use a struct to create a tuple
    struct Color(i32, i32, i32, i32);
    let black = Color(0, 0, 0, 1);
    println!("{} {} {} {}", black.0, black.1, black.2, black.3);

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                &self.width * &self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let r = Rectangle {
            width: 32,
            height: 32
        };

        let r2 = Rectangle {
            width:16,
            height: 16
        };

        println!("{} {} {:?}", r.can_hold(&r2), r.area(), r);
    }
}
