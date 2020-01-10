use indexmap::IndexMap;
use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};
#[cfg(feature = "serde")]
mod der;
mod display;
mod parser;
#[cfg(feature = "serde")]
mod ser;

#[derive(Debug, PartialEq, Eq)]
pub struct Bibliography {
    entry_type: String,
    citation_key: String,
    tags: IndexMap<String, String>,
}

impl Default for Bibliography {
    fn default() -> Self {
        Self { entry_type: String::new(), citation_key: String::new(), tags: IndexMap::new() }
    }
}

impl Bibliography {
    pub fn authors(&self) -> Vec<&str> {
        match self.tags.get("author") {
            Some(s) => s.split("and").map(|s| s.trim()).collect(),
            None => {
                vec![]
            }
        }
    }
}
