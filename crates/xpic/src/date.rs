use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::de::Visitor;
use serde::{self, Deserializer, Serializer};
use std::fmt::Formatter;

/// Serde format for `NaiveDate` as `YYYYMMDD`.
pub mod ymd {
    use super::*;

    const FORMAT: &str = "%Y%m%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(YMDVisitor)
    }

    struct YMDVisitor;

    impl Visitor<'_> for YMDVisitor {
        type Value = NaiveDate;

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "a date string in YYYYMMDD format")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            NaiveDate::parse_from_str(v, FORMAT).map_err(E::custom)
        }
    }
}

/// Serde format for `DateTime<Utc>` as `YYYYMMDDHHmm`.
pub mod ymdhm {
    use super::*;

    const FORMAT: &str = "%Y%m%d%H%M";

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(YMDHMVisitor)
    }

    struct YMDHMVisitor;

    impl Visitor<'_> for YMDHMVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "a datetime string in YYYYMMDDHHmm format")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            NaiveDateTime::parse_from_str(v, FORMAT)
                .map(|naive| naive.and_utc())
                .map_err(E::custom)
        }
    }
}
