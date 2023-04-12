use std::collections::HashSet;

use paperclip::v2::{models::DataType, models::DefaultSchemaRaw};
use strum::{Display, EnumString};

/// The universe of scopes that can be used thoughout the application.
#[derive(Debug, Clone, EnumString, Display, PartialEq, Eq, Hash)]
pub enum Scope {
    #[strum(serialize = "urn:zita:scope:example")]
    Example,
}

/// A unique set of scopes, being defined as [`Scope`].
#[derive(Debug, Default, Clone)]
pub struct Scopes(HashSet<Scope>);

impl<'de> serde::Deserialize<'de> for Scopes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Ok(Self(
            string
                .split(' ')
                .map(str::trim)
                .filter(|scope| !scope.is_empty())
                .map(std::str::FromStr::from_str)
                .collect::<Result<_, _>>()
                .map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for Scopes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            &self
                .0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

impl std::ops::Deref for Scopes {
    type Target = HashSet<Scope>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl paperclip::v2::schema::Apiv2Schema for Scopes {
    fn raw_schema() -> paperclip::v2::models::DefaultSchemaRaw {
        DefaultSchemaRaw {
            data_type: Some(DataType::String),
            ..Default::default()
        }
    }
}
