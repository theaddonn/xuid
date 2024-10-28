use std::fmt::{Debug, Display, Formatter};
use std::num::{NonZeroU64, ParseIntError, TryFromIntError};
use std::str::FromStr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Deserializer, Serializer};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Xuid(NonZeroU64);

impl TryFrom<u64> for Xuid {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        value.try_into().map(Xuid)
    }
}

impl From<Xuid> for u64 {
    #[inline]
    fn from(x: Xuid) -> Self {
        x.0.get()
    }
}

impl From<Xuid> for String {
    #[inline]
    fn from(x: Xuid) -> Self {
        x.0.to_string()
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

impl FromStr for Xuid {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse().map(Self)
    }
}

impl Display for Xuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0.get())
    }
}

impl Display for Xuid {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.get())
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

#[macro_export]
macro_rules! xuid {
    ($xuid:literal) => {
        xuid::Xuid::try_from($xuid)
    };
}
