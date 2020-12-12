use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de, Deserialize, Deserializer};

pub fn deserialize_datetime_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let timestamp = s.parse::<i64>().map_err(de::Error::custom)?;

    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    Ok(DateTime::from_utc(naive_datetime, Utc))
}
