use crate::imports::*;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Base58 decode error: {0}")]
    Base58Decode(#[from] bs58::decode::Error),
    #[error("invalid identifier value '{0}'")]
    Invalid(String),
}

pub const ID_SIZE: usize = 32;

#[repr(transparent)]
#[derive(
    Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize,
)]
pub struct Id(pub(crate) [u8; ID_SIZE]);

impl Id {
    pub const fn zero() -> Id {
        Id([0; ID_SIZE])
    }

    pub fn new() -> Id {
        Id::new_from_slice(&rand::random::<[u8; ID_SIZE]>())
    }

    pub fn new_from_slice(vec: &[u8]) -> Self {
        Self(
            <[u8; ID_SIZE]>::try_from(<&[u8]>::clone(&vec))
                .expect("Error: invalid slice size for id"),
        )
    }

    pub fn try_new_from_slice(vec: &[u8]) -> Result<Self> {
        Ok(Self(<[u8; ID_SIZE]>::try_from(<&[u8]>::clone(&vec))?))
    }

    pub fn to_bytes(self) -> [u8; ID_SIZE] {
        self.0
    }

    pub fn composite(&self, other: &str) -> Self {
        let data = [self.as_ref(), other.as_bytes()].concat();
        let hash = Hash::sha256(data.as_slice());
        hash.into()
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.to_string()
    }
}

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Id {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() > std::mem::size_of::<Id>() * 2 {
            return Err(Error::Invalid(s.to_string()));
        }
        let vec = bs58::decode(s).into_vec()?;
        if vec.len() != std::mem::size_of::<Id>() {
            Err(Error::Invalid(s.to_string()))
        } else {
            Ok(Id::new_from_slice(&vec))
        }
    }
}

impl TryFrom<&str> for Id {
    type Error = Error;
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        Id::from_str(s)
    }
}

// impl TryFrom<JsValue> for Id {
//     type Error = Error;
//     fn try_from(value: JsValue) -> std::result::Result<Self, Self::Error> {
//         let value_str = value.as_string().ok_or(Error::JsValueNotString)?;
//         FromStr::from_str(&value_str)
//     }
// }

// impl From<Id> for JsValue {
//     fn from(id: Id) -> Self {
//         JsValue::from_str(&id.to_string())
//     }
// }

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::string::String as Deserialize>::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub trait IdT {
    fn id(&self) -> Id;
}
