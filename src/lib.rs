use crate::front_of_room::hosting as host;

mod front_of_room;

pub fn eat_at_restaurant() {
    host::add_to_waitlist();
    host::add_to_waitlist();
    host::add_to_waitlist();
    host::add_to_waitlist();
}