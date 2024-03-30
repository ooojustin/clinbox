use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    // #[deprecated(since = "0.4.29", note = "use `DateTime::parse_from_str` instead")]
    //Utc.datetime_from_str(s, FORMAT).map_err(serde::de::Error::custom)

    match NaiveDateTime::parse_from_str(s, FORMAT) {
        Ok(dt) => Ok(Utc.from_utc_datetime(&dt)),
        Err(_) => Err(serde::de::Error::custom(
            "Failed to parse DateTime from string.",
        )),
    }
}
