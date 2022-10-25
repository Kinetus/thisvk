use super::vk_date_format;
use chrono::naive::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(value: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'a>(#[serde(with = "vk_date_format")] &'a NaiveDate);

    value.as_ref().map(Helper).serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "vk_date_format")] NaiveDate);

    let helper = Option::deserialize(deserializer)?;
    Ok(helper.map(|Helper(external)| external))
}
