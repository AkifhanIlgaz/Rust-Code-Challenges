/*
    Has a deadline been reached ?

    Create a function that can tell if an event's date has already occurred

*/

// TODO: import the necessary dependencies

use chrono::{Local, TimeZone};

struct ImportantEvent {
    // TODO: define data structure
    what: String,
    when: chrono::Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    // TODO: implement trait
    fn is_passed(&self) -> bool {
        Local::today() > self.when
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.ymd(2020, 12, 25),
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}
