use chrono::NaiveDate;
use serde::{Deserializer, Serializer, Deserialize};
use serde::de::{Error, Unexpected};

const FORMAT: &'static str = "%F";

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>
{
    let date = <&str>::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date, FORMAT)
        .ok()
        .ok_or(D::Error::invalid_value(
            Unexpected::Str(&date),
            &"Expected a yyyy-mm-dd date string",
        ))
}

pub(crate) fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let date_str: String = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&date_str)
}
