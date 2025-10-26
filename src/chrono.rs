use chrono::{Datelike, NaiveDate, Utc};

pub type Date = NaiveDate;

/// 13th January, 2005
pub const BIRTHDAY: Date = Date::from_ymd_opt(2005, 1, 13).unwrap();

pub fn age_from(birthday: Date) -> i32 {
    let date = today();

    let mut age = date.year() - birthday.year();

    if date.month() < birthday.month()
        || date.month() == birthday.month() && date.day() < birthday.day()
    {
        age -= 1;
    }

    age
}

pub fn age() -> i32 {
    age_from(BIRTHDAY)
}

pub fn today() -> Date {
    Utc::now().date_naive()
}

pub fn year() -> i32 {
    today().year()
}
