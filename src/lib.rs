use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Deserializer, Serializer};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Xuid(u64);

impl From<u64> for Xuid {
    #[inline]
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl From<Xuid> for u64 {
    #[inline]
    fn from(x: Xuid) -> Self {
        x.0
    }
}

impl From<Xuid> for String {
    #[inline]
    fn from(x: Xuid) -> Self {
        x.0.to_string()
    }
}

impl From<Xuid> for &str {
    fn from(x: Xuid) -> Self {

    }
}

impl TryFrom<String> for Xuid {
    type Error = ParseIntError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}

impl TryFrom<&str> for Xuid {
    type Error = ParseIntError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}

impl Display for Xuid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Xuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Xuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}
