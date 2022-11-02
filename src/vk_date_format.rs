use chrono::naive::NaiveDate;
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%d.%m.%Y";

pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    
    let mut result = NaiveDate::parse_from_str(&s, FORMAT);

    if result.is_err() {
        result = NaiveDate::parse_from_str(&format!("{}.0000", s), FORMAT);
    }
    
    result.map_err(serde::de::Error::custom)
}
