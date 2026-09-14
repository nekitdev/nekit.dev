use jiff::{Zoned, civil};

pub type Date = civil::Date;

/// 13th January, 2005 is my birthday! ~ nekit
pub const BIRTHDAY: Date = Date::constant(2005, 1, 13);

pub fn age_from(birthday: Date) -> i16 {
    let date = today();

    let mut age = date.year() - birthday.year();

    if date.month() < birthday.month()
        || date.month() == birthday.month() && date.day() < birthday.day()
    {
        age -= 1;
    }

    age
}

pub fn age() -> i16 {
    age_from(BIRTHDAY)
}

pub fn today() -> Date {
    Zoned::now().date()
}

pub fn year() -> i16 {
    today().year()
}
