use super::*;
use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::str::FromStr;

pub struct BibliographyVisitor<'i> {
    ptr: &'i mut Bibliography,
}

impl<'de> Deserialize<'de> for Bibliography {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut empty = Bibliography::default();
        deserializer.deserialize_any(BibliographyVisitor { ptr: &mut empty })?;
        Ok(empty)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BibliographyVisitor { ptr: place })
    }
}

impl<'i, 'de> Visitor<'de> for BibliographyVisitor<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except a bibtex bibliography")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Bibliography::from_str(v).map(|b| *self.ptr = b).map_err(|e| Error::custom(e))
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Ok(Some(s)) = map.next_key::<String>() {
            match s.as_str() {
                "entry_type" => {
                    self.ptr.entry_type = map.next_value()?;
                }
                "citation_key" => {
                    self.ptr.citation_key = map.next_value()?;
                }
                _ => {
                    let value = map.next_value()?;
                    self.ptr.tags.insert(s, value);
                }
            }
        }
        Ok(())
    }
}
