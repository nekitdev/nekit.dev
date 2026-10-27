use jiff::civil;
use serde::{Deserialize, Deserializer, de::Error};
use toml::value::Date as TomlDate;

pub type Date = civil::Date;

pub fn required<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
    let toml = TomlDate::deserialize(deserializer)?;

    let date = toml.to_string().parse().map_err(D::Error::custom)?;

    Ok(date)
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Wrap {
    #[serde(deserialize_with = "required")]
    date: Date,
}

impl Wrap {
    const fn extract(self) -> Date {
        self.date
    }
}

type OptionWrap = Option<Wrap>;

pub fn optional<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Date>, D::Error> {
    let option = OptionWrap::deserialize(deserializer)?.map(Wrap::extract);

    Ok(option)
}
