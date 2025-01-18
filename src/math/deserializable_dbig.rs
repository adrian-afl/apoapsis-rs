use dashu_float::DBig;
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DeserializableDBig(pub DBig);

impl<'de> Deserialize<'de> for DeserializableDBig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeserializableDBigVisitor;
        impl<'de> Visitor<'de> for DeserializableDBigVisitor {
            type Value = DeserializableDBig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string DBig")
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let dbig = DBig::from_str(v.as_str())
                    .map_err(|_| E::custom(format!("failed to parse {} as DBig", v)))?;

                Ok(DeserializableDBig(dbig))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let dbig = DBig::from_str(v)
                    .map_err(|_| E::custom(format!("failed to parse {} as DBig", v)))?;

                Ok(DeserializableDBig(dbig))
            }
        }
        deserializer.deserialize_string(DeserializableDBigVisitor)
    }
}
