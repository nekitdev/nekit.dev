use chrono::{Datelike, NaiveDate, Utc};

pub type Date = NaiveDate;

/// 13th January, 2005
pub const NEKIT_BIRTHDAY: Date = Date::from_ymd_opt(2005, 1, 13).unwrap();

pub fn get(birthday: Date) -> i32 {
    let today = Utc::now().date_naive();

    let mut age = today.year() - birthday.year();

    if today.month() < birthday.month()
        || (today.month() == birthday.month() && today.day() < birthday.day())
    {
        age -= 1;
    }

    age
}

pub fn nekit() -> i32 {
    get(NEKIT_BIRTHDAY)
}
