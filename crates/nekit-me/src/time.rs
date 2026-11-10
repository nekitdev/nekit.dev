use jiff::{
    Timestamp, Zoned,
    tz::{Offset, TimeZone},
};
use schemars::JsonSchema;
use serde::Serialize;

pub const OFFSET: Offset = Offset::constant(3); // UTC+3

pub const TIME_ZONE: TimeZone = TimeZone::fixed(OFFSET);

pub const RFC_3339: &str = "%Y-%m-%dT%H:%M:%S%:z";
pub const WEEK_DAY: &str = "%A";

pub fn now() -> Zoned {
    let timestamp = Timestamp::now();

    timestamp.to_zoned(TIME_ZONE)
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct View {
    /// RFC 3339 date and time with offset, e.g. `2005-01-13T13:42:22+03:00`.
    pub date_time: String,
    /// Week day, e.g. `Thursday`.
    pub week_day: String,
}

impl View {
    pub fn new(zoned: Zoned) -> Self {
        let date_time = zoned.strftime(RFC_3339).to_string();
        let week_day = zoned.strftime(WEEK_DAY).to_string();

        Self {
            date_time,
            week_day,
        }
    }
}

pub fn view() -> View {
    let zoned = now();

    View::new(zoned)
}
