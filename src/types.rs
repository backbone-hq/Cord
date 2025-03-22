use crate::{CordError, CordResult};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes(pub(crate) Vec<u8>);

impl Bytes {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(vector: Vec<u8>) -> Self {
        Bytes(vector)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.0
    }
}

impl From<&Bytes> for Vec<u8> {
    fn from(bytes: &Bytes) -> Self {
        bytes.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Set<T> {
    pub hashset: HashSet<T>,
}

impl<T> PartialEq for Set<T>
where
    HashSet<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hashset == other.hashset
    }
}

impl<T: Clone> From<&Set<T>> for Vec<T> {
    fn from(set: &Set<T>) -> Self {
        set.hashset.iter().cloned().collect()
    }
}

impl<T> From<HashSet<T>> for Set<T> {
    fn from(hashset: HashSet<T>) -> Self {
        Self { hashset }
    }
}

impl<T> From<Vec<T>> for Set<T>
where
    T: Hash + Eq,
{
    fn from(vector: Vec<T>) -> Self {
        Set::from_iter(vector)
    }
}

impl<T: Hash + Eq> FromIterator<T> for Set<T> {
    fn from_iter<E: IntoIterator<Item = T>>(iter: E) -> Self {
        Set::from(HashSet::from_iter(iter))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct DateTime {
    pub chrono: chrono::DateTime<chrono::Utc>,
}

impl DateTime {
    pub fn now() -> Self {
        Self {
            chrono: chrono::Utc::now(),
        }
    }
}

impl FromStr for DateTime {
    type Err = CordError;

    fn from_str(s: &str) -> CordResult<Self, Self::Err> {
        let chrono = chrono::DateTime::<chrono::Utc>::from_str(s)
            .map_err(|_| CordError::ValidationError("Failed to parse datetime"))?;
        Ok(Self { chrono })
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(chrono: chrono::DateTime<chrono::Utc>) -> Self {
        Self { chrono }
    }
}
