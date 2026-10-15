use jiff::{
    Timestamp, Zoned, civil,
    tz::{Offset, TimeZone},
};

pub type Date = civil::Date;
pub type Time = civil::Time;

/// 13th January, 2005 is my birthday! ~ nekit
pub const BIRTHDAY: Date = Date::constant(2005, 1, 13);

/// Moscow's time zone is UTC+3, no DST since 2014
pub const TIME_ZONE: TimeZone = TimeZone::fixed(Offset::constant(3));

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

pub fn zoned() -> Zoned {
    Timestamp::now().to_zoned(TIME_ZONE)
}

pub fn today() -> Date {
    zoned().date()
}

pub fn year() -> i16 {
    today().year()
}

pub fn now() -> Time {
    zoned().time()
}

pub const CLOCK: &str = "%H:%M";

pub fn clock() -> String {
    now().strftime(CLOCK).to_string()
}
