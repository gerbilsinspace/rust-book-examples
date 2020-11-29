mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}

fn main() {
    eat_at_restaurant();
}
